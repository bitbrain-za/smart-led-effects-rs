use palette::Srgb;

pub struct Wipe {
    position: usize,
    buffer: Vec<Srgb<u8>>,
    reverse: bool,
    end: usize,
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
        }
    }

    pub fn colour_wipe(count: usize, colour: Srgb<u8>, reverse: bool) -> Self {
        let colour: Vec<Srgb<u8>> = vec![colour; count];
        Wipe::new(count, colour, reverse)
    }
}

impl Iterator for Wipe {
    type Item = Vec<Srgb<u8>>;
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let out = self
            .buffer
            .iter()
            .skip(self.position)
            .take(self.buffer.len() / 3)
            .copied()
            .collect::<Vec<Srgb<u8>>>();

        if self.reverse {
            self.position -= 1;
            if self.position == 0 {
                self.position = self.end;
            }
        } else {
            self.position += 1;
            if self.position >= self.end {
                self.position = 0;
            }
        }

        Some(out)
    }
}
