use gif::{Frame, Encoder, Repeat};
use std::fs::File;
use std::borrow::Cow;
 
fn main() {
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let (width, height) = (6, 6);
    let beacon_states = [[
        0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
    ], [
        0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
    ]];
    let mut image = File::create("beacon.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for state in &beacon_states {
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}
