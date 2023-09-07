use std::fmt::Display;
pub mod strip;
mod utils;
use palette::{FromColor, Hsv, IntoColor, Srgb};

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "red: {}, green: {}, blue: {}",
            self.red, self.green, self.blue
        )
    }
}

impl FromColor<Srgb<u8>> for Rgb {
    fn from_color(color: Srgb<u8>) -> Self {
        Rgb {
            red: color.red,
            green: color.green,
            blue: color.blue,
        }
    }
}

impl FromColor<Srgb> for Rgb {
    fn from_color(color: Srgb) -> Self {
        Rgb {
            red: (color.red * 255.0) as u8,
            green: (color.green * 255.0) as u8,
            blue: (color.blue * 255.0) as u8,
        }
    }
}

impl FromColor<Hsv> for Rgb {
    fn from_color(color: Hsv) -> Self {
        Rgb::from_color(Srgb::from_color(color))
    }
}

impl IntoColor<Srgb<u8>> for Rgb {
    fn into_color(self) -> Srgb<u8> {
        Srgb::new(self.red, self.green, self.blue)
    }
}

impl IntoColor<Srgb> for Rgb {
    fn into_color(self) -> Srgb {
        Srgb::new(
            self.red as f32 / 255.0,
            self.green as f32 / 255.0,
            self.blue as f32 / 255.0,
        )
    }
}

impl IntoColor<Hsv> for Rgb {
    fn into_color(self) -> Hsv {
        let srgb: Srgb = self.into_color();
        Hsv::from_color(srgb)
    }
}
