use crate::rgb::Rgb;
pub type ImagePixels = Vec<Vec<Rgb>>;

#[derive(Debug, Clone)]
pub struct Image {
    pub pixels: ImagePixels,
    pub path: String,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(path: String) -> Self {
        Self {
            pixels: Vec::new(),
            path,
            width: 0,
            height: 0,
        }
    }
}