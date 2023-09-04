use palette::{FromColor, Hsv, Srgb};

pub fn hsv_to_srgb(hsv: Vec<Hsv>) -> Vec<Srgb<u8>> {
    hsv.iter()
        .map(|x| Srgb::from_color(*x).into_format::<u8>())
        .collect::<Vec<Srgb<u8>>>()
}

pub fn single_hsv_to_srgb(hsv: Hsv) -> Srgb<u8> {
    Srgb::from_color(hsv).into_format::<u8>()
}
