use palette::{FromColor, Hsv, Srgb};

pub fn srgbu8_to_hsv(input: Srgb<u8>) -> Hsv {
    Hsv::from_color(input.into_format())
}
