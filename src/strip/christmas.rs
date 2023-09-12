use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, MixAssign, Srgb};
use rand::{thread_rng, Rng};

use crate::utils::{hsv_to_srgb, srgbu8_to_hsv};

pub struct Christmas {
    frequency: u8,
    probability: f32,
    fade: f32,
    current: Vec<Hsv>,
    mix: Vec<f32>,
    background: Hsv,
}

impl Christmas {
    const DEFAULT_FREQUENCY: u8 = 0x04;
    const DEFAULT_PROBABILITY: f32 = 0.1;
    const DEFAULT_FADE: f32 = 0.4;
    const BASE_BRIGHTNESS: f32 = 1.0;
    const BACKGROUND: Srgb = Srgb::new(0.0, 1.0, 0.0);
    const BASE_MIX: f32 = -100.0;

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
            current: vec![colour; count],
            mix: vec![0.0; count],
            background: colour,
        }
    }
}

impl Christmas {
    fn generate_sparkle(&mut self) {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.current.len());

        let c_index = rng.gen_range(0.0..1.0);

        let colour = if c_index < 0.5 {
            Hsv::from_color(Srgb::new(1.0, 0.0, 0.0))
        } else if c_index < 0.80 {
            Hsv::from_color(Srgb::new(0.0, 0.84, 1.0))
        } else {
            Hsv::from_color(Srgb::new(0.0, 0.0, 1.0))
        };

        let mut sparkle = colour;
        sparkle.value = rng.gen_range(0.7..1.0);

        let chance = rng.gen_range(0.0..1.0);
        if chance < self.probability {
            self.current[index] = sparkle;
            self.mix[index] = -Self::BASE_MIX;
        }
    }

    fn fade_sparkles(&mut self) {
        for (i, pixel) in self.current.iter_mut().enumerate() {
            self.mix[i] += self.fade;
            self.mix[i] = self.mix[i].min(1.0);
            pixel.mix_assign(self.background, self.mix[i].max(1.0));
        }
    }
}

impl EffectIterator for Christmas {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        self.fade_sparkles();

        let chances = thread_rng().gen_range(0..self.frequency);
        for _ in 0..chances {
            self.generate_sparkle();
        }

        Some(hsv_to_srgb(self.current.clone()))
    }
}
