use crate::strip::{EffectIterator, Wipe};
use palette::Srgb;

pub struct Morse {
    wipe: Wipe,
}

impl Morse {
    pub fn new(count: usize, message: &str, colour: Option<Srgb<u8>>, reverse: bool) -> Self {
        let code = Self::string_to_morse(message);

        let colour = colour.unwrap_or(Srgb::new(255, 0, 0));

        let code = code
            .iter()
            .map(|&x| if x == 1 { colour } else { Srgb::new(0, 0, 0) })
            .collect::<Vec<Srgb<u8>>>();

        let wipe = Wipe::new(count, code, reverse);

        Morse { wipe }
    }

    fn string_to_morse(message: &str) -> Vec<u8> {
        let mut out = Vec::new();
        for c in message.chars() {
            match c {
                'a' | 'A' => out.extend(vec![1, 0, 1, 1, 1]),
                'b' | 'B' => out.extend(vec![1, 1, 1, 0, 1, 0, 1, 0, 1]),
                'c' | 'C' => out.extend(vec![1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1]),
                'd' | 'D' => out.extend(vec![1, 1, 1, 0, 1, 0, 1]),
                'e' | 'E' => out.extend(vec![1, 1, 1, 0]),
                'f' | 'F' => out.extend(vec![1, 0, 1, 0, 1, 1, 1, 0, 1]),
                'g' | 'G' => out.extend(vec![1, 1, 1, 0, 1, 1, 1, 0, 1]),
                'h' | 'H' => out.extend(vec![1, 0, 1, 0, 1, 0, 1]),
                'i' | 'I' => out.extend(vec![1, 0, 1]),
                'j' | 'J' => out.extend(vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1]),
                'k' | 'K' => out.extend(vec![1, 1, 1, 0, 1, 0, 1, 1, 1]),
                'l' | 'L' => out.extend(vec![1, 0, 1, 1, 1, 0, 1, 0, 1]),
                'm' | 'M' => out.extend(vec![1, 1, 1, 0, 1, 1, 1]),
                'n' | 'N' => out.extend(vec![1, 1, 1, 0, 1]),
                'o' | 'O' => out.extend(vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1]),
                'p' | 'P' => out.extend(vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1]),
                'q' | 'Q' => out.extend(vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1]),
                'r' | 'R' => out.extend(vec![1, 0, 1, 1, 1, 0, 1]),
                's' | 'S' => out.extend(vec![1, 0, 1, 0, 1]),
                't' | 'T' => out.extend(vec![1, 0]),
                'u' | 'U' => out.extend(vec![1, 0, 1, 0, 1, 1, 1]),
                'v' | 'V' => out.extend(vec![1, 0, 1, 0, 1, 0, 1, 1, 1]),
                'w' | 'W' => out.extend(vec![1, 0, 1, 1, 1, 0, 1, 1, 1]),
                'x' | 'X' => out.extend(vec![1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]),
                'y' | 'Y' => out.extend(vec![1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1]),
                'z' | 'Z' => out.extend(vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1]),
                '0' => out.extend(vec![
                    1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1,
                ]),
                '1' => out.extend(vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1]),
                '2' => out.extend(vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1]),
                '3' => out.extend(vec![1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1]),
                '4' => out.extend(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1]),
                '5' => out.extend(vec![1, 0, 1, 0, 1, 0, 1, 0, 1]),
                '6' => out.extend(vec![1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1]),
                '7' => out.extend(vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1]),
                '8' => out.extend(vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1]),
                '9' => out.extend(vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1]),
                _ => (),
            }
            out.extend(vec![0, 0, 0]);
        }
        out
    }
}

impl EffectIterator for Morse {
    fn name(&self) -> &'static str {
        "Morse"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        self.wipe.next()
    }
}
