use crate::strip::EffectIterator;
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
    const DEFAULT_SIZE: usize = 4;
    const DEFAULT_FADE: f32 = 0.2;

    pub fn new(count: usize, colour: Srgb<u8>, size: Option<usize>, fade: Option<f32>) -> Self {
        let mut brightness = vec![0.0; count];
        let size = size.unwrap_or(Cylon::DEFAULT_SIZE);

        for pixel in brightness.iter_mut().take(size) {
            *pixel = 1.0;
        }

        Cylon {
            colour: Hsv::from_color(colour.into_format()),
            brightness,
            start: size - 1,
            direction: Direction::Forward,
            size,
            fade: fade.unwrap_or(Cylon::DEFAULT_FADE),
        }
    }
}

impl EffectIterator for Cylon {
    fn name(&self) -> &'static str {
        "Cylon"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let len = self.brightness.len();

        let mut trail: Vec<f32> = vec![0.0; len];

        match self.direction {
            Direction::Forward => {
                self.brightness.rotate_right(1);
                self.start += 1;

                if self.start == len - 1 {
                    self.direction.next();
                }

                let mut val: f32 = 0.8;
                if self.start + 1 > self.size {
                    for i in (0..self.start + 1 - self.size).rev() {
                        trail[i] = val;
                        val = (val - self.fade).max(0.0);
                    }
                }
            }
            _ => {
                self.brightness.rotate_left(1);
                self.start -= 1;
                if self.start == self.size - 1 {
                    self.direction.next();
                }

                if self.start < len - 1 {
                    let mut val: f32 = 0.8;
                    for pixel in trail.iter_mut().take(len).skip(self.start) {
                        *pixel = val;
                        val = (val - self.fade).max(0.0);
                    }
                }
            }
        };

        let out: Vec<Srgb<u8>> = self
            .brightness
            .iter()
            .zip(trail.iter())
            .map(|(&x, &y)| {
                let mut pixel = self.colour;
                pixel.value = (x + y).min(1.0);
                Srgb::from_color(pixel).into_format::<u8>()
            })
            .collect();

        Some(out)
    }
}
