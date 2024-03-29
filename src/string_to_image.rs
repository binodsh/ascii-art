use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn ascii_to_image(text: &Vec<String>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let font = Vec::from(include_bytes!("UbuntuMono-R.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let char_height = 6.0;
    let scale = Scale {
        x: char_height * 3.0,
        y: char_height * 2.0,
    };

    let buffer_width = (char_height as u32) * 3 * (text.get(0).unwrap().len() as u32) / 2;
    let buffer_height = (char_height as u32) * 2 * (text.len() as u32);

    let mut image_buffer = RgbImage::new(buffer_width, buffer_height);

    let color: Rgb<u8> = Rgb { 0: [255, 255, 255] };

    println!("starting to draw......");
    for (i, line) in text.iter().enumerate() {
        draw_text_mut(
            &mut image_buffer,
            color,
            0,
            (i * 12) as u32,
            scale,
            &font,
            line,
        );
    }

    image_buffer
}
