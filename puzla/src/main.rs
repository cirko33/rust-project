// Luka Ciric E2 18/2023
pub mod loader;
pub mod my_image;
pub mod rgb;
pub mod get_max;

use std::{thread, sync::{Arc, Mutex}};

use get_max::get_max_wh;
use loader::{load_images, load_main_image};
use my_image::Image;
use rgb::Rgb;

fn get_diff(a: Rgb, b: Rgb) -> f64 {
    (a.r as f64 - b.r as f64).abs() + (a.g as f64 - b.g as f64).abs() + (a.b as f64 - b.b as f64).abs()
}

fn get_factor(main_image: &Image, image: &Image, start_w: usize, start_h: usize) -> f64 {
    let mut fact = 0.0;
    let local_max_w = if start_w + image.width < main_image.width { image.width } else { main_image.width - start_w };
    let local_max_h = if start_h + image.height < main_image.height { image.height } else { main_image.height - start_h };

    for j in 0..local_max_h {
        for i in 0..local_max_w {
            let main_pixel = main_image.pixels[start_h + j][start_w + i];
            let pixel = image.pixels[j][i];
            fact +=  get_diff(main_pixel, pixel);
        }
    }

    fact / (local_max_w * local_max_h) as f64
}

type Factor = (usize, usize, f64);
type ImageWithCoords = (Image, usize, usize);

fn main() {
    let images = load_images("./slike/slika 1/");
    let main_image = Arc::new(load_main_image("./slike/picture1.jpg"));

    let (max_w, max_h) = get_max_wh(&images);
    let grid_w = main_image.width / max_w + if (main_image.width % max_w) > 5 { 1 } else { 0 };
    let grid_h = main_image.height / max_h + if (main_image.height % max_h) > 5 { 1 } else { 0 };

    let mut handles = Vec::new();
    let best_factors: Arc<Mutex<Vec<ImageWithCoords>>> = Arc::new(Mutex::new(Vec::new()));
    for im in images.clone() {
        let m_img = main_image.clone();
        let (grid_wc, grid_hc, max_wc, max_hc) = (grid_w.clone(), grid_h.clone(), max_w.clone(), max_h.clone());
        let best_factors_clone = best_factors.clone();
        let handle = thread::spawn(move|| {
            let mut best_factor: Factor = (0, 0, f64::MAX);
            for j in 0..grid_hc {
                for i in 0..grid_wc {
                    let factor = get_factor(&m_img, &im, i * max_wc, j * max_hc);
                    if factor < best_factor.2 {
                        best_factor = (i, j, factor);
                    }
                }
            }
            best_factors_clone.lock().unwrap().push((im, best_factor.0, best_factor.1));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut best_factors = Arc::try_unwrap(best_factors).unwrap().into_inner().unwrap();
    best_factors.sort_by(|a, b|  (a.2 * grid_h + a.1).cmp(&(b.2 * grid_h + b.1)));
    for f in best_factors {
        println!("{} {} {}", f.0.path, f.1, f.2);
    }
}
