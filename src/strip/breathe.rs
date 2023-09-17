use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, IntoColor, Srgb};
use rand::{thread_rng, Rng};

enum Direction {
    Up,
    Down,
}
pub struct Breathe {
    colour: Hsv,
    random_colour: bool,
    direction: Direction,
    count: usize,
    step: f32,
}

impl Breathe {
    const DEFAULT_STEP: f32 = 0.02;
    pub fn new(count: usize, colour: Option<Srgb>, step_size: Option<f32>) -> Self {
        let random_colour = colour.is_none();
        let colour: Hsv = match colour {
            Some(colour) => colour.into_color(),
            None => Hsv::new(0.0, 1.0, 1.0),
        };

        let mut me = Breathe {
            colour,
            random_colour,
            direction: Direction::Up,
            count,
            step: step_size.unwrap_or(Self::DEFAULT_STEP),
        };
        me.set_colour();
        me
    }

    fn set_colour(&mut self) {
        if self.random_colour {
            let mut rng = thread_rng();
            self.colour = Hsv::new(rng.gen_range(0.0..360.0), 1.0, 1.0);
        }
        self.colour.value = 0.0;
    }
}

impl EffectIterator for Breathe {
    fn name(&self) -> &'static str {
        "Breathe"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        match self.direction {
            Direction::Up => {
                self.colour.value += self.step;
                if self.colour.value >= 1.0 {
                    self.direction = Direction::Down;
                }
            }
            Direction::Down => {
                self.colour.value -= self.step;
                if self.colour.value <= 0.0 {
                    self.direction = Direction::Up;
                    self.set_colour();
                }
            }
        };

        Some(vec![
            Srgb::from_color(self.colour).into_format::<u8>();
            self.count
        ])
    }
}
