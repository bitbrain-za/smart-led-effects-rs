use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, Srgb};
use rand::{thread_rng, Rng};

use crate::utils::hsv_to_srgb;

pub struct Twinkle {
    frequency: u8,
    probability: f32,
    fade: f32,
    colour: Option<Hsv>,
    current: Vec<Hsv>,
}

impl Twinkle {
    const DEFAULT_FREQUENCY: u8 = 0x04;
    const DEFAULT_PROBABILITY: f32 = 0.1;
    const DEFAULT_FADE: f32 = 0.02;
    pub fn new(
        count: usize,
        colour: Option<Srgb<u8>>,
        sparkle: Option<u8>,
        probability: Option<f32>,
        fade: Option<f32>,
    ) -> Self {
        Twinkle {
            frequency: sparkle.unwrap_or(Twinkle::DEFAULT_FREQUENCY),
            fade: fade.unwrap_or(Twinkle::DEFAULT_FADE),
            probability: probability.unwrap_or(Twinkle::DEFAULT_PROBABILITY),
            current: vec![Hsv::new(0.0, 1.0, 0.0); count],
            colour: colour.map(|colour| Hsv::from_color(colour.into_format())),
        }
    }

    pub fn sparkle(count: usize, colour: Option<Srgb<u8>>) -> Self {
        let colour = match colour {
            Some(colour) => Some(colour),
            None => Some(Srgb::<u8>::new(255, 255, 255)),
        };
        Twinkle::new(count, colour, Some(20), Some(0.4), Some(1.0))
    }
}

impl Twinkle {
    fn generate_sparkle(&mut self) {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.current.len());

        let mut sparkle = match self.colour {
            Some(colour) => colour,
            None => Hsv::new(rng.gen_range(0.0..360.0), 1.0, 0.0),
        };

        sparkle.value = rng.gen_range(0.5..1.0);

        let chance = rng.gen_range(0.0..1.0);
        if chance < self.probability {
            self.current[index] = sparkle;
        }
    }

    fn fade_sparkles(&mut self) {
        for pixel in self.current.iter_mut() {
            pixel.value = if pixel.value > self.fade {
                pixel.value - self.fade
            } else {
                0.0
            };
        }
    }
}

impl EffectIterator for Twinkle {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        self.fade_sparkles();

        let chances = thread_rng().gen_range(0..self.frequency);
        for _ in 0..chances {
            self.generate_sparkle();
        }

        Some(hsv_to_srgb(self.current.clone()))
    }
}
