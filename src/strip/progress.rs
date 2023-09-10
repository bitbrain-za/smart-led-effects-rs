use palette::{Mix, Srgb};

pub struct ProgressBar {
    count: usize,
    start_colour: Srgb,
    end_colour: Srgb,
    gradient: bool,
    pixels_per_percent: f32,
}

impl ProgressBar {
    const DEFAULT_START_COLOUR: Srgb = Srgb::new(0.0, 0.0, 1.0);
    const DEFAULT_END_COLOUR: Srgb = Srgb::new(1.0, 0.0, 0.0);
    pub fn new(
        count: usize,
        start_colour: Option<Srgb>,
        end_colour: Option<Srgb>,
        gradient: Option<bool>,
    ) -> Self {
        ProgressBar {
            count,
            start_colour: start_colour.unwrap_or(Self::DEFAULT_START_COLOUR),
            end_colour: end_colour.unwrap_or(Self::DEFAULT_END_COLOUR),
            gradient: gradient.unwrap_or(false),
            pixels_per_percent: count as f32 / 100.0,
        }
    }

    pub fn get_output_for_value(&mut self, percentage: f32) -> Vec<Srgb<u8>> {
        let pixels = self.count - (self.pixels_per_percent * percentage) as usize;
        let mut out = vec![Srgb::new(0.0, 0.0, 0.0).into_format(); self.count];
        let percentage = percentage.max(0.0).min(100.0) / 100.0;

        if self.gradient {
            for (i, pixel) in out.iter_mut().take(pixels).enumerate() {
                *pixel = self
                    .end_colour
                    .mix(self.start_colour, i as f32 / self.count as f32)
                    .into_format();
            }
        } else {
            for pixel in out.iter_mut().take(pixels) {
                *pixel = self
                    .end_colour
                    .mix(self.end_colour, percentage)
                    .into_format();
            }
        }

        out
    }
}
