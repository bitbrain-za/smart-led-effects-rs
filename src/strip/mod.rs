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

pub const LIST: &[&str] = &[
    "Breathe",
    "Bounce",
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

pub enum Effect {
    Breathe(Breathe),
    Bounce(Bounce),
    Cycle(Cycle),
    Cylon(Cylon),
    Fire(Fire),
    Meteor(Meteor),
    ProgressBar(ProgressBar),
    Rainbow(Rainbow),
    RunningLights(RunningLights),
    Timer(Timer),
    Twinkle(Twinkle),
    SnowSparkle(SnowSparkle),
    Wipe(Wipe),
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Breathe(_) => write!(f, "Breathe"),
            Effect::Bounce(_) => write!(f, "Bounce"),
            Effect::Cycle(_) => write!(f, "Cycle"),
            Effect::Cylon(_) => write!(f, "Cylon"),
            Effect::Fire(_) => write!(f, "Fire"),
            Effect::Meteor(_) => write!(f, "Meteor"),
            Effect::ProgressBar(_) => write!(f, "ProgressBar"),
            Effect::Rainbow(_) => write!(f, "Rainbow"),
            Effect::RunningLights(_) => write!(f, "RunningLights"),
            Effect::Timer(_) => write!(f, "Timer"),
            Effect::Twinkle(_) => write!(f, "Twinkle"),
            Effect::SnowSparkle(_) => write!(f, "SnowSparkle"),
            Effect::Wipe(_) => write!(f, "Wipe"),
        }
    }
}
