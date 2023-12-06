// Adapted from Rustdoc example.

use gifski::{self, Repeat, Settings, progress::NoProgress};
use imgref::{Img, ImgVec};
use rgb::RGBA8;

fn make_img(dim: usize, phase: bool) -> ImgVec<RGBA8> {
    let black = RGBA8 { r: 0, g: 0, b: 0, a: 255 };
    let red = RGBA8 { r: 255, g: 0, b: 0, a: 255 };
    let pixels = vec![black; dim * dim];
    let mut img = Img::new(pixels, dim, dim);
    let mut make_rect = |x0, y0| {
        let mut region = img.sub_image_mut(x0, y0, dim / 2, dim / 2);
        for p in region.pixels_mut() {
            *p = red;
        }
    };
    match phase {
        false => {
            make_rect(0, 0);
            make_rect(dim / 2, dim / 2);
        }
        true => {
            make_rect(dim / 2, 0);
            make_rect(0, dim / 2);
        }
    }

    img
}

fn main() {
    let dim = 128;
    let mut settings = Settings::default();
    settings.repeat = Repeat::Infinite;
    let (encoder, writer) = gifski::new(settings).unwrap();

    for phase in [false, true] {
        let img = make_img(dim as usize, phase);
        let i = phase as usize;
        encoder.add_frame_rgba(i, img, i as f64 * 0.5).unwrap();
    }
    drop(encoder);

    let gif = std::fs::File::create("flasher.gif").unwrap();
    let mut no_progress = NoProgress {};
    writer.write(gif, &mut no_progress).unwrap();

}
