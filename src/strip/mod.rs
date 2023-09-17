mod breathe;
pub use breathe::Breathe;
mod bounce;
pub use bounce::Bounce;
mod christmas;
pub use christmas::Christmas;
mod collision;
pub use collision::Collision;
mod cycle;
pub use cycle::Cycle;
mod cylon;
pub use cylon::Cylon;
mod fire;
pub use fire::Fire;
mod meteor;
pub use meteor::Meteor;
mod progress;
pub use progress::ProgressBar;
mod rainbow;
pub use rainbow::Rainbow;
mod running_lights;
pub use running_lights::RunningLights;
mod timer;
pub use timer::Timer;
mod twinkle;
pub use twinkle::Twinkle;
mod snow_sparkle;
pub use snow_sparkle::SnowSparkle;
mod wipe;
pub use wipe::Wipe;

mod effects_trait;
pub use effects_trait::EffectIterator;

const LIST: &[&str] = &[
    "Breathe",
    "Bounce",
    "Collision",
    "Cycle",
    "Cylon",
    "Fire",
    "Meteor",
    "ProgressBar",
    "Rainbow",
    "RunningLights",
    "Timer",
    "Twinkle",
    "SnowSparkle",
    "Wipe",
];

pub fn list() -> Vec<String> {
    LIST.iter().map(|s| s.to_string()).collect()
}

pub fn get_default_effect(count: usize, name: &str) -> Option<Box<dyn EffectIterator>> {
    match name {
        "Breathe" => Some(Box::new(Breathe::new(count, None, None))),
        "Bounce" => Some(Box::new(Bounce::new(count, None, None, None, None, None))),
        "Collision" => Some(Box::new(Collision::new(count, Some(true)))),
        "Cycle" => Some(Box::new(Cycle::new(count, None))),
        "Cylon" => Some(Box::new(Cylon::new(
            count,
            palette::Srgb::<u8>::new(255, 0, 0),
            None,
            None,
        ))),
        "Fire" => Some(Box::new(Fire::new(count, None, None))),
        "Meteor" => Some(Box::new(Meteor::new(count, None, None, None))),
        // "ProgressBar" => Some(Box::new(ProgressBar::new(count, None, None))),
        "Rainbow" => Some(Box::new(Rainbow::new(count, None))),
        "RunningLights" => Some(Box::new(RunningLights::new(count, None, false))),
        // "Timer" => Some(Box::new(Timer::new(count, None, None))),
        "Twinkle" => Some(Box::new(Twinkle::new(count, None, None, None, None))),
        "SnowSparkle" => Some(Box::new(SnowSparkle::new(count, None, None, None, None))),
        "Wipe" => Some(Box::new(Wipe::new(count, vec![], false))),
        _ => None,
    }
}

pub fn get_all_default_effects(count: usize) -> Vec<Box<dyn EffectIterator>> {
    let mut effects = Vec::new();
    for name in LIST {
        if let Some(effect) = get_default_effect(count, name) {
            effects.push(effect);
        }
    }
    effects
}
