use image::{Delay, DynamicImage, Frame, Rgb, RgbImage, RgbaImage};
use image::codecs::gif::{GifEncoder, Repeat};

fn make_rgba(dim: u32, phase: bool) -> RgbaImage {
    let mut rgb = RgbImage::new(dim, dim);
    let mut make_rect = |x0, y0| {
        for x in x0..x0 + dim / 2 {
            for y in y0..y0 + dim / 2 {
                rgb.put_pixel(x, y, Rgb([255, 0, 0]));
            }
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
    DynamicImage::ImageRgb8(rgb).into_rgba8()
}

fn main() {
    let outfile = std::fs::File::create("flasher.gif").unwrap();
    let mut encoder = GifEncoder::new(outfile);
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let delay = Delay::from_numer_denom_ms(500, 1);
    for phase in [false, true] {
        let rgba = make_rgba(128, phase);
        let frame = Frame::from_parts(rgba, 0, 0, delay);
        encoder.encode_frame(frame).unwrap();
    }
}
