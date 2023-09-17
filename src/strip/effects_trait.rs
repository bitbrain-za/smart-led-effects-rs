use palette::Srgb;

pub trait EffectIterator {
    fn name(&self) -> &'static str;
    fn next(&mut self) -> Option<Vec<Srgb<u8>>>;
}
