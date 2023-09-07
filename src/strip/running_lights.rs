use crate::strip::Effect;
use crate::utils::{single_hsv_to_srgb, srgbu8_to_hsv};
use palette::{Hsv, Srgb};
pub struct RunningLights {
    colour: Hsv,
    count: usize,
    position: usize,
    reverse: bool,
}

impl RunningLights {
    pub fn new(count: usize, colour: Option<Srgb<u8>>, reverse: bool) -> Self {
        RunningLights {
            colour: match colour {
                Some(colour) => srgbu8_to_hsv(colour),
                None => Hsv::new(0.0, 0.0, 1.0),
            },
            count,
            position: match reverse {
                true => count,
                false => 0,
            },
            reverse,
        }
    }
}

impl Effect for RunningLights {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let mut out = vec![Srgb::<u8>::new(0, 0, 0); self.count];
        for (i, pixel) in out.iter_mut().enumerate() {
            let brightness = (i as f32 + self.position as f32).sin() / 2.0 + 0.5;
            let mut hsv = self.colour;
            hsv.value = brightness;
            *pixel = single_hsv_to_srgb(hsv);
        }
        if self.reverse {
            self.position -= 1;
            if self.position == 0 {
                self.position = self.count;
            }
        } else {
            self.position += 1;
            if self.position >= self.count {
                self.position = 0;
            }
        }

        Some(out)
    }
}
