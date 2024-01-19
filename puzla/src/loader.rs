use std::{fs, thread, sync::{Arc, Mutex}};
use image::{GenericImageView, RgbImage};    
use crate::{my_image::{ImagePixels, Image}, rgb::Rgb};


pub fn load_images(folder_path: &str) -> Vec<Image> {
    let images: Arc<Mutex<Vec<Image>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let cloned_images = Arc::clone(&images);
                let handle = thread::spawn(move || {
                    let file_path = entry.path();
                    let image = load_image(file_path.to_str().unwrap());
                    cloned_images.lock().unwrap().push(image);
                });
                handles.push(handle);
            }
        }
    } else {
        panic!("Error: Could not read folder path");
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    return Arc::try_unwrap(images).unwrap().into_inner().unwrap();
}

fn load_image(file_path: &str) -> Image {
    let mut image = Image::new(file_path.to_string().clone());
    if let Ok(img) = image::open(file_path) {
        let rgb_image: RgbImage = img.to_rgb8();
        let (width, height) = img.dimensions();
        image.width = width as usize;
        image.height = height as usize;

        for y in 0..height {
            let mut row: Vec<Rgb> = Vec::new();
            for x in 0..width {
                let pixel = rgb_image.get_pixel(x, y);
                row.push(Rgb::new(pixel[0], pixel[1], pixel[2]));
            }
            image.pixels.push(row);
        }
    } else {
        panic!("Error: Could not open image");
    }

    image
}

pub fn load_main_image(file_path: &str) -> Image {
    let mut image = Image::new(file_path.to_string().clone());
    if let Ok(img) = image::open(file_path) {
        let rgb_image: RgbImage = img.to_rgb8();
        let (width, height) = img.dimensions();
        image.width = width as usize;
        image.height = height as usize;
        
        for y in 0..height {
            let mut row: Vec<Rgb> = Vec::new();
            for x in 0..width {
                let pixel = rgb_image.get_pixel(x, y);
                row.push(Rgb::new(pixel[0], pixel[1], pixel[2]));
            }
            image.pixels.push(row);
        }
    } else {
        panic!("Error: Could not open image");
    }

    image
}