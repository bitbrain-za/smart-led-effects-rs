use core::panic;
use std::vec;

use palette::{FromColor, Hsv, Srgb};

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn next(&mut self) {
        match self {
            Direction::Forward => *self = Direction::Backward,
            _ => *self = Direction::Forward,
        }
    }
}

pub struct Cylon {
    colour: Hsv,
    direction: Direction,
    brightness: Vec<f32>,
    start: usize,
    size: usize,
    fade: f32,
}

impl Cylon {
    const DEFAULT_SIZE: usize = 1;
    const DEFAULT_FADE: f32 = 0.0;

    pub fn new(count: usize, colour: Srgb<u8>, size: Option<usize>, fade: Option<f32>) -> Self {
        let mut brightness = vec![0.0; count];
        let size = size.unwrap_or(Cylon::DEFAULT_SIZE);

        for i in 0..size {
            brightness[i] = 1.0;
        }

        Cylon {
            colour: Hsv::from_color(colour.into_format()),
            brightness,
            start: size,
            direction: Direction::Forward,
            size,
            fade: fade.unwrap_or(Cylon::DEFAULT_FADE),
        }
    }
}

impl Cylon {
    fn reset(&mut self) {
        self.brightness = vec![0.0; self.brightness.len()];
        for i in 0..self.size {
            self.brightness[i] = 1.0;
        }
    }
}

impl Iterator for Cylon {
    type Item = Vec<Srgb<u8>>;
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let len = self.brightness.len();

        if self.start == len - self.size + 1 {
            self.start = self.size;
            self.reset();
            self.direction.next();
        } else {
            self.start += 1;
            self.brightness.rotate_right(1);
        }

        let mut out: Vec<Srgb<u8>> = self
            .brightness
            .iter()
            .map(|x| {
                let mut pixel = self.colour;
                pixel.value = *x;
                Srgb::from_color(pixel).into_format::<u8>()
            })
            .collect();

        if self.direction == Direction::Backward {
            out.reverse();
        }

        Some(out)
    }
}
