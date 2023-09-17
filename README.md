# Neopixel Effects

This crate borrows heavily from [fastLED](https://github.com/FastLED/FastLED) and [tweaking4all](https://www.tweaking4all.com/hardware/arduino/adruino-led-strip-effects/).

## Dimensionality

Currently only works for strips/loops. But someday the plan is to extend it.

## Effects

    - Breathe
    - Bounce
    - Collision
    - Cylon
    - Fire
    - Meteor
    - ProgressBar
    - Rainbow
    - RunningLights
    - Timer
    - Twinkle
    - SnowSparkle
    - Wipe

## Example Usage

```toml
[dependencies]
smart_led_effects = 0.1.0

```

```rust

use smart_led_effects::{
    strip::{self, EffectIterator},
    Srgb,
};

//...

    const COUNT: usize = 55;
    let effect = strip::Rainbow::new(COUNT, None);

    loop {
        let pixels = effect.next().unwrap();
    
        // show pixels

        thread::sleep(Duration::from_millis(10));
    }


```

## References

 - [Palette](https://crates.io/crates/palette)
 - fastLED



