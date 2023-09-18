use palette::{FromColor, Hsv, Srgb};
use rand::Rng;
use std::time::{Duration, Instant};

use super::EffectIterator;

/// Strobe effect
///
/// This effect flashes the whole strip in a given colour or random colour if None is supplied.
///
/// # Arguments
///
/// * `count` - The number of pixels in the strip.
/// * `colour` - The colour to flash. If `None` a random colour will be used.
/// * `period` - The period of the strobe.
/// * `decay` - The rate at which the colour fades. If `None` the default value of `0.02` per tick will be used.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use smart_led_effects::strip::Strobe;
///
/// let count = 10;
/// let colour = None;
/// let period = Duration::from_secs(1);
/// let decay = None;
///
/// let mut effect = Strobe::new(count, colour, period, decay);
/// ```
#[derive(Debug)]
pub struct Strobe {
    count: usize,
    colour: Option<Hsv>,
    current_colour: Hsv,
    period: Duration,
    fade_val: f32,
    start: Instant,
}

impl Strobe {
    pub fn new(
        count: usize,
        colour: Option<Srgb<u8>>,
        period: Duration,
        decay: Option<f32>,
    ) -> Self {
        let colour = colour.map(|c| Hsv::from_color(c.into_format::<f32>()));
        let current_colour = match colour {
            Some(colour) => colour,
            None => Hsv::new(0.0, 0.0, 1.0),
        };

        Strobe {
            count,
            colour,
            current_colour,
            period,
            fade_val: decay.unwrap_or(0.02),
            start: Instant::now(),
        }
    }

    fn genereate_colour(&mut self) {
        let mut rng = rand::thread_rng();
        self.current_colour = Hsv::new(rng.gen_range(0.0..360.0), rng.gen_range(0.0..1.0), 1.0);
    }

    fn fade(&mut self) -> bool {
        self.current_colour.value -= self.fade_val;
        if self.current_colour.value <= 0.0 {
            self.current_colour.value = 0.0;
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        match self.colour {
            Some(colour) => self.current_colour = colour,
            None => self.genereate_colour(),
        }
        self.current_colour.value = 1.0;
        self.start = Instant::now();
    }
}

impl EffectIterator for Strobe {
    fn name(&self) -> &'static str {
        "Strobe"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        if self.fade() {
            let elapsed = self.start.elapsed().as_secs();
            if elapsed >= self.period.as_secs() {
                self.reset();
            }
        }
        let out = vec![Srgb::from_color(self.current_colour).into_format::<u8>(); self.count];
        Some(out)
    }
}
