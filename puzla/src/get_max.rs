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