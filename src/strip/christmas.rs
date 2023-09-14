use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, MixAssign, Srgb};
use rand::{thread_rng, Rng};

use crate::utils::srgbu8_to_hsv;

pub struct Sparkle {
    colour: Srgb,
    intensity: f32,
    location: usize,
}

pub struct Christmas {
    frequency: u8,
    probability: f32,
    fade: f32,
    sparkles: Vec<Sparkle>,
    length: usize,
}

impl Christmas {
    const DEFAULT_FREQUENCY: u8 = 0x04;
    const DEFAULT_PROBABILITY: f32 = 0.1;
    const DEFAULT_FADE: f32 = 0.4;
    const BASE_BRIGHTNESS: f32 = 1.0;
    const BACKGROUND: Srgb = Srgb::new(6.0 / 255.0, 108.0 / 255.0, 22.0 / 255.0);

    pub fn new(
        count: usize,
        colour: Option<Srgb<u8>>,
        sparkle: Option<u8>,
        probability: Option<f32>,
        fade: Option<f32>,
    ) -> Self {
        let mut colour = match colour {
            Some(colour) => srgbu8_to_hsv(colour),
            None => Hsv::from_color(Self::BACKGROUND),
        };

        colour.value = Christmas::BASE_BRIGHTNESS;

        Christmas {
            frequency: sparkle.unwrap_or(Christmas::DEFAULT_FREQUENCY),
            fade: fade.unwrap_or(Christmas::DEFAULT_FADE),
            probability: probability.unwrap_or(Christmas::DEFAULT_PROBABILITY),
            sparkles: Vec::new(),
            length: count,
        }
    }
}

impl Christmas {
    fn generate_sparkle(&mut self) {
        let mut rng = thread_rng();

        let chance = rng.gen_range(0.0..1.0);
        if chance > self.probability {
            return;
        }

        let index = rng.gen_range(0..self.length);

        let c_index = rng.gen_range(0.0..1.0);

        let colour = if c_index < 0.5 {
            Srgb::new(1.0, 0.0, 0.0)
        } else if c_index < 0.80 {
            Srgb::new(0.0, 0.84, 1.0)
        } else {
            Srgb::new(0.0, 0.0, 1.0)
        };

        self.sparkles.push(Sparkle {
            colour,
            intensity: 1.0,
            location: index,
        });
    }

    fn fade_sparkles(&mut self) {
        for sparkle in self.sparkles.iter_mut() {
            sparkle.intensity -= self.fade;
            sparkle.intensity = sparkle.intensity.max(0.0);
        }
        self.sparkles.retain(|sparkle| sparkle.intensity > 0.0);
    }
}

impl EffectIterator for Christmas {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        self.fade_sparkles();

        let chances = thread_rng().gen_range(0..self.frequency);
        for _ in 0..chances {
            self.generate_sparkle();
        }

        let mut out: Vec<Srgb> = vec![self::Christmas::BACKGROUND; self.length];

        for sparkle in self.sparkles.iter() {
            out[sparkle.location].mix_assign(sparkle.colour, sparkle.intensity);
        }

        Some(out.iter().map(|&c| c.into_format::<u8>()).collect())
    }
}
