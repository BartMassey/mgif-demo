// Adapted from Rustdoc example.

use gif::{Frame, Encoder, Repeat};
 
fn make_frame(dim: usize, phase: bool) -> Frame<'static> {
    let mut buffer = vec![0u8; dim * dim];
    let mut make_rect = |x0, y0| {
        for x in x0..x0 + dim / 2 {
            for y in y0..y0 + dim / 2 {
                buffer[x + y * dim] = 1;
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

    let mut frame = Frame::default();
    frame.width = dim as u16;
    frame.height = dim as u16;
    frame.buffer = std::borrow::Cow::Owned(buffer);
    frame.delay = 50;
    frame
}

fn main() {
    let color_map = &[0, 0, 0, 0xff, 0, 0];
    let dim = 128;
    let mut image = std::fs::File::create("flasher.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, dim, dim, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for phase in [false, true] {
        let frame = make_frame(dim as usize, phase);
        encoder.write_frame(&frame).unwrap();
    }
}
