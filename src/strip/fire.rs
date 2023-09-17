use crate::strip::EffectIterator;
use palette::Srgb;
use rand::{thread_rng, Rng};

pub struct Fire {
    cooling: u8,
    sparking: u8,
    heat: Vec<u8>,
}

impl Fire {
    const DEFAULT_COOLING: u8 = 40;
    const DEFAULT_SPARKING: u8 = 120;
    pub fn new(count: usize, cooling: Option<u8>, sparking: Option<u8>) -> Self {
        Fire {
            cooling: (((cooling.unwrap_or(Fire::DEFAULT_COOLING) as f32 * 10.0) / count as f32)
                + 2.0) as u8,
            sparking: sparking.unwrap_or(Fire::DEFAULT_SPARKING),
            heat: vec![0; count],
        }
    }
}

impl Fire {
    fn heat_to_colour(val: u8) -> Srgb<u8> {
        if val >= 0x85 {
            let heat_ramp = 3u8.saturating_mul(val - 0x85);
            Srgb::new(255, 255, heat_ramp)
        } else if val >= 0x40 {
            let heat_ramp = 3u8.saturating_mul(val - 0x40);
            Srgb::new(255, heat_ramp, 0)
        } else {
            let heat_ramp = 3u8.saturating_mul(val);
            Srgb::new(heat_ramp, 0, 0)
        }
    }
}

impl EffectIterator for Fire {
    fn name(&self) -> &'static str {
        "Fire"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let mut rng = thread_rng();

        /* apply cooling */
        for spark in self.heat.iter_mut() {
            let x = rng.gen_range(0..self.cooling) as u8;
            *spark = spark.saturating_sub(x);
        }

        /* apply heating */
        for i in (2..self.heat.len()).rev() {
            self.heat[i] = (self.heat[i - 1]
                .saturating_add(self.heat[i - 2])
                .saturating_add(self.heat[i - 2]))
                / 3;
        }

        /* generate sparks */
        if rng.gen_range(0..255) < self.sparking {
            let y = rng.gen_range(0..self.heat.len() / 7);
            self.heat[y] = self.heat[y].saturating_add(rng.gen_range(160..255));
        }

        Some(
            self.heat
                .iter()
                .map(|x| Fire::heat_to_colour(*x))
                .collect::<Vec<Srgb<u8>>>(),
        )
    }
}
