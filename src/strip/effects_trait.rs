use palette::Srgb;

pub trait EffectIterator {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>>;
}
