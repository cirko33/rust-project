use crate::my_image::Image;

pub fn get_max_wh(images: &Vec<Image>) -> (usize, usize) {
    let mut max_w = 0;
    let mut max_h = 0;

    for image in images {
        if image.width > max_w {
            max_w = image.width;
        }
        if image.height > max_h {
            max_h = image.height;
        }
    }

    (max_w, max_h)
}

pub fn get_grid_wh(main_image: &Image, max_w: usize, max_h: usize) -> (usize, usize) {
    let grid_w = main_image.width / max_w + if (main_image.width % max_w) > 5 { 1 } else { 0 };
    let grid_h = main_image.height / max_h + if (main_image.height % max_h) > 5 { 1 } else { 0 };

    (grid_w, grid_h)
}