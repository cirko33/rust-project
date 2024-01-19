// Luka Ciric E2 18/2023
pub mod loader;
pub mod my_image;
pub mod rgb;
pub mod get_max;

use std::{thread, sync::Arc};

use get_max::get_max_wh;
use loader::{load_images, load_main_image};

fn main() {
    let images = load_images("./slike/slika 1/");
    let main_image = Arc::new(load_main_image("./slike/picture1.jpg"));

    let (max_w, max_h) = get_max_wh(&images);
    let grid_w = main_image.width / max_w + if (main_image.width % max_w) > 5 { 1 } else { 0 };
    let grid_h = main_image.height / max_h + if (main_image.height % max_h) > 5 { 1 } else { 0 };

    let mut handles = Vec::new();

    for im in images.clone() {
        let m_img = main_image.clone();
        let handle = thread::spawn(move|| {
            for j in 0..grid_h {
                for i in 0..grid_w {
                    
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
