//! A library that generates effects for use with addressable LEDs.
//! Currently it only implements 1 dimensional effects for use with a single strip of LEDs.
//!
//! # Usage
//!
//! Each effect is implemented as an iterator that returns a vector of [Srgb] colours.
//! The vector returned is the same length as the number of LEDs in the strip (supplied when instantiating the effect).
//! This vector can then be passed to the LED strip driver.
//!
//! The [strip::EffectIterator] trait is implemented for each effect, so they can be used as iterators. The iterators will currently all loop forever.
//!
//! # Effects
//! | Name | Description |
//! | ---- | ----------- |
//! | [strip::Bounce] | The bounce effect will generate a number of balls that bounce up and down the strip |
//! | [strip::Breathe] | The breathe effect will generate a single colour that fades in and out |
//! | [strip::Christmas] | (WIP) Has a green background with random red, blue and gold sparkles |
//! | [strip::Collision] | Generates two particles that can collide and bounce or shatter |
//! | [strip::Cycle] | Rotates around the HSV colour space |
//! | [strip::Cylon] | Generates the cylon eye effect |
//! | [strip::Fire] | Generates an effect like a flickering flame |
//! | [strip::Meteor] | Generates a meteor that goes down the strip trailing bits of fading debris |
//! | [strip::ProgressBar] | (WIP) Signals progress
//! | [strip::Rainbow] | Generates a rainbow effect |
//! | [strip::RunningLights] | Generates a running lights effect |
//! | [strip::SnowSparkle] | Generates random sparkles |
//! | [strip::Strobe] | Strobe light/blinder effect |
//! | [strip::Timer] | Counts down for the given duration |
//! | [strip::Twinkle] | Generates random twinkles |
//! | [strip::Wipe] | Generates a wipe effect |
//!
//!
//! # Example
//!
//! ```rust
//! use smart_led_effects::{
//!     strip::{self, EffectIterator},
//!     Srgb,
//! };
//!
//!
//! const COUNT: usize = 55;
//! let effect = strip::Rainbow::new(COUNT, None);
//!
//! {
//! let pixels = effect.next().unwrap();
//!    
//! // show pixels
//!
//! thread::sleep(Duration::from_millis(10));
//! }
//!
pub mod strip;
mod utils;
pub use palette::Srgb;
