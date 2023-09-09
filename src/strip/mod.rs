mod breathe;
pub use breathe::Breathe;
mod bounce;
pub use bounce::Bounce;
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
pub use effects_trait::Effect;

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

pub enum Effects {
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

impl std::fmt::Display for Effects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effects::Breathe(_) => write!(f, "Breathe"),
            Effects::Bounce(_) => write!(f, "Bounce"),
            Effects::Cycle(_) => write!(f, "Cycle"),
            Effects::Cylon(_) => write!(f, "Cylon"),
            Effects::Fire(_) => write!(f, "Fire"),
            Effects::Meteor(_) => write!(f, "Meteor"),
            Effects::ProgressBar(_) => write!(f, "ProgressBar"),
            Effects::Rainbow(_) => write!(f, "Rainbow"),
            Effects::RunningLights(_) => write!(f, "RunningLights"),
            Effects::Timer(_) => write!(f, "Timer"),
            Effects::Twinkle(_) => write!(f, "Twinkle"),
            Effects::SnowSparkle(_) => write!(f, "SnowSparkle"),
            Effects::Wipe(_) => write!(f, "Wipe"),
        }
    }
}
