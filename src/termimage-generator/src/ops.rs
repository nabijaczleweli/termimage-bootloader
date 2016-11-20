use image::{GenericImage, DynamicImage, FilterType, Pixel};
use termimage::util::closest_colour;
use std::ops::Index;


pub fn fname_to_8_3(fname: &str) -> String {
    fname.trim()
        .to_uppercase()
        .chars()
        .filter(|&c| {
            (c as u32 >= 128 && c as u32 <= 255) ||
            ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2',
             '3', '4', '5', '6', '7', '8', '9']
                .contains(&c)
        })
        .take(8)
        .collect()
}

pub fn resize(img: &DynamicImage) -> DynamicImage {
    img.resize_exact(320, 200, FilterType::CatmullRom)
}

pub fn create_colourtable<C: Index<usize, Output = u8>>(img: &DynamicImage, colours: &[C]) -> Vec<Vec<usize>> {
    let (width, height) = img.dimensions();

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| closest_colour(img.get_pixel(x, y).to_rgb(), colours))
                .collect()
        })
        .collect()
}
