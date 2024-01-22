// Luka Ciric E2 18/2023
pub mod loader;
pub mod my_image;
pub mod rgb;
pub mod wh;
pub mod find_mins;
pub mod factor;
pub mod write_in_file;

use find_mins::find_mins_and_execute;
use wh::{get_max_wh, get_grid_wh};
use factor::{get_factors, sort_factors};
use loader::{load_images, load_main_image};
use write_in_file::write_factors;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Not enough arguments provided.");
    }

    let puzzle_path = format!("./slike/slika {}/", args[1].to_string());
    let main_image_path = format!("./slike/picture{}.jpg", args[2].to_string());
    
    let images = load_images(&puzzle_path);
    let main_image = load_main_image(&main_image_path);

    let (max_w, max_h) = get_max_wh(&images);
    let (grid_w, grid_h) = get_grid_wh(&main_image, max_w, max_h);

    let mut factors = get_factors(images, &main_image, grid_w, grid_h, max_w, max_h);
    sort_factors(&mut factors, grid_h, grid_w);

    let ret = find_mins_and_execute(&mut factors, grid_h, grid_w);
    write_factors(ret, max_w, max_h);
}
