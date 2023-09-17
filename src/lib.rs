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
