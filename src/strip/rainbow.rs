use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, ShiftHueAssign, Srgb};

pub struct Rainbow {
    last_state: Vec<Hsv>,
    step_size: f32,
}

impl Rainbow {
    pub fn new(count: usize, steps: Option<usize>) -> Self {
        let mut last_state = Vec::new();
        let mut color = Hsv::new(0.0, 1.0, 1.0);
        last_state.push(color);
        let separation = 360.0 / count as f32;
        let step = steps.unwrap_or(360);
        let step_size = 360.0 / step as f32;

        for _ in 1..count {
            color.shift_hue_assign(separation);
            last_state.push(color);
        }
        Rainbow {
            last_state,
            step_size,
        }
    }
}

impl EffectIterator for Rainbow {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        for pixel in self.last_state.iter_mut() {
            pixel.shift_hue_assign(self.step_size);
        }

        Some(
            self.last_state
                .iter()
                .map(|x| Srgb::from_color(*x).into_format::<u8>())
                .collect::<Vec<Srgb<u8>>>(),
        )
    }
}
