# Smart LED Effects

This supplies a collection of effects for usage with individually addressable LED strips such as the WS2812b.
Each effect returns a vector of colours that can then be sent to your LED driver.

The EffectIterator trait defines two methods:
    - `name`
    - `next`

`name` will just return the name as a static string slice.
`next` will return the next page of the effect. It uses the `Option` enum, and in the future there will be One Shot effects that end and return `None`. For now, all effects will loop.

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
    - SnowSparkle
    - Strobe
    - Timer
    - Twinkle
    - Wipe

## Example Usage

```toml
[dependencies]
smart_led_effects = 0.1.6

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



