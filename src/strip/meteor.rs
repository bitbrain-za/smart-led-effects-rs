use crate::strip::EffectIterator;
use palette::{Darken, Srgb};
use rand::{thread_rng, Rng};
pub struct Meteor {
    count: usize,
    colour: Srgb,
    size: usize,
    position: usize,
    fade: f32,
    current: Vec<Srgb>,
    random_colour: bool,
}

impl Meteor {
    const DEFAULT_SIZE: usize = 4;
    const DEFAULT_FADE: f32 = 0.3;
    const DEFAULT_COLOUR: Srgb<u8> = Srgb::<u8>::new(255, 255, 255);

    pub fn new(
        count: usize,
        colour: Option<Srgb<u8>>,
        size: Option<usize>,
        fade: Option<f32>,
    ) -> Self {
        Meteor {
            count,
            colour: colour.unwrap_or(Self::DEFAULT_COLOUR).into_format(),
            size: size.unwrap_or(Self::DEFAULT_SIZE),
            position: 0,
            fade: fade.unwrap_or(Self::DEFAULT_FADE),
            current: vec![Srgb::new(0.0, 0.0, 0.0); count],
            random_colour: colour.is_none(),
        }
    }
}

impl EffectIterator for Meteor {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let mut rng = thread_rng();
        for pixel in self.current.iter_mut() {
            if rng.gen_range(0.0..1.0) < 0.5 {
                *pixel = pixel.darken(self.fade);
            }
        }

        for i in 0..self.size {
            if (self.position.saturating_sub(i) < self.count) && (self.position >= i) {
                self.current[self.position - i] = self.colour;
            }
        }
        self.position += 1;
        if self.position > 2 * self.count {
            if self.random_colour {
                self.colour = Srgb::new(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                );
            }
            self.position = 0;
        }

        Some(self.current.iter().map(|p| p.into_format()).collect())
    }
}
