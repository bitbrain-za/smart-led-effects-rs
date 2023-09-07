use palette::Srgb;

pub trait Effect {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>>;
}
