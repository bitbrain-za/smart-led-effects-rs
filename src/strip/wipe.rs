use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, Srgb};
use rand::Rng;

pub struct Wipe {
    position: usize,
    buffer: Vec<Srgb<u8>>,
    reverse: bool,
    end: usize,
    count: usize,
    randomize: bool,
}

impl Wipe {
    pub fn new(count: usize, data: Vec<Srgb<u8>>, reverse: bool) -> Self {
        let mut buffer = vec![Srgb::<u8>::new(0, 0, 0); count];
        buffer.extend(data);
        buffer.extend(vec![Srgb::<u8>::new(0, 0, 0); count]);

        let end = buffer.len() - count;

        Wipe {
            position: match reverse {
                true => end,
                false => 0,
            },
            buffer,
            reverse,
            end,
            count,
            randomize: false,
        }
    }

    pub fn colour_wipe(count: usize, colour: Option<Srgb<u8>>, reverse: bool) -> Self {
        let mut s = Wipe::new(count, vec![Srgb::new(0, 0, 0); count], reverse);
        match colour {
            Some(colour) => s.fill_wipe(colour),
            None => s.randomize_colour_wipe(),
        }
        s
    }

    fn fill_wipe(&mut self, colour: Srgb<u8>) {
        let mut buffer = vec![Srgb::<u8>::new(0, 0, 0); self.count];
        buffer.extend(vec![colour; self.count]);
        buffer.extend(vec![Srgb::<u8>::new(0, 0, 0); self.count]);
        self.buffer = buffer;
    }

    fn randomize_colour_wipe(&mut self) {
        let mut rng = rand::thread_rng();
        let colour: Srgb<u8> =
            Srgb::from_color(Hsv::new(rng.gen_range(0.0..360.0), 1.0, 1.0)).into_format();
        self.fill_wipe(colour);
        self.randomize = true;
    }
}

impl EffectIterator for Wipe {
    fn name(&self) -> &'static str {
        "Wipe"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let out = self
            .buffer
            .iter()
            .skip(self.position)
            .take(self.count)
            .copied()
            .collect::<Vec<Srgb<u8>>>();

        if self.reverse {
            self.position -= 1;
            if self.position == 0 {
                self.position = self.end;
                if self.randomize {
                    self.randomize_colour_wipe();
                }
            }
        } else {
            self.position += 1;
            if self.position >= self.end {
                self.position = 0;
                if self.randomize {
                    self.randomize_colour_wipe();
                }
            }
        }
        Some(out)
    }
}
