use std::{thread, sync::{Mutex, Arc}};

use crate::{my_image::Image, rgb::Rgb};

pub type Factor = (String, f64);
pub type PictureMap = Vec<Vec<Vec<Factor>>>;

pub fn get_factors(images: Vec<Image>, main_image: &Image, grid_w: usize, grid_h: usize, max_w: usize, max_h: usize) -> PictureMap {    
    let mut handles = Vec::new();
    let factors: Arc<Mutex<PictureMap>> = Arc::new(Mutex::new(vec![vec![Vec::new(); grid_w]; grid_h]));
    for im in images {
        let m_img = main_image.clone();
        let (grid_wc, grid_hc, max_wc, max_hc) = (grid_w.clone(), grid_h.clone(), max_w.clone(), max_h.clone());
        let factors_clone = factors.clone();
        
        let handle = thread::spawn(move|| {
            let mut local_factors = vec![vec![Vec::new(); grid_w]; grid_h];
            for i in 0..grid_hc {
                for j in 0..grid_wc {
                    let factor = get_factor(&m_img, &im, j * max_wc, i * max_hc);
                    local_factors[i][j].push((im.path.clone(), factor));
                }
            }
            
            {
                let mut fc = factors_clone.lock().unwrap();
                for i in 0..grid_hc {
                    for j in 0..grid_wc {
                        fc[i][j].append(&mut local_factors[i][j]);
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(factors).unwrap().into_inner().unwrap()
}

fn get_factor(main_image: &Image, image: &Image, start_w: usize, start_h: usize) -> f64 {
    let mut fact = 0.0;
    let local_max_w = if start_w + image.width < main_image.width { image.width } else { main_image.width - start_w };
    let local_max_h = if start_h + image.height < main_image.height { image.height } else { main_image.height - start_h };

    for i in 0..local_max_h {
        for j in 0..local_max_w {
            let main_pixel = main_image.pixels[start_h + i][start_w + j];
            let pixel = image.pixels[i][j];
            fact += get_diff(main_pixel, pixel);
        }
    }

    fact / (local_max_w * local_max_h) as f64
}

fn get_diff(a: Rgb, b: Rgb) -> f64 {
    (a.r as f64 - b.r as f64).abs() + (a.g as f64 - b.g as f64).abs() + (a.b as f64 - b.b as f64).abs()
}

pub fn sort_factors(factors: &mut PictureMap, grid_h: usize, grid_w: usize) {
    for i in 0..grid_h{
        for j in 0..grid_w {
            factors[i][j].sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        }
    }
}