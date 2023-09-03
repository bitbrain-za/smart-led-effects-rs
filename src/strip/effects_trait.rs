use palette::Srgb;

pub trait Effect {
    // fn new(pixels: usize, steps: Option<usize>) -> Self;
    fn next(&mut self) -> Option<Vec<Srgb<u8>>>;
}
