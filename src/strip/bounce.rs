use crate::strip::Effect;
use palette::{Darken, FromColor, Hsv, Srgb};
use rand::{thread_rng, Rng};
use std::{ops::Range, time::Instant};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Ball {
    position: f32,
    speed: f32, /* pixels per a second */
    colour: Srgb,
    direction: Direction,
    last_update: Instant,
    random_colour: bool,
    gravity: f32,
    bounciness: Range<f32>,
    speed_range: Range<f32>,
    current_bounciness: f32,
}

impl Ball {
    const DEFAULT_GRAVITY: f32 = 30.0; // pixels per a second ^ 2
    const DEFAULT_BOUNCINESS: Range<f32> = 0.2..0.8;
    const DEFAULT_SPEEDS: Range<f32> = 20.0..80.0;
    fn new(
        colour: Option<Srgb>,
        gravity: Option<f32>,
        bounciness: Option<Range<f32>>,
        speed: Option<Range<f32>>,
    ) -> Self {
        let mut rng = thread_rng();
        Ball {
            position: 0.0,
            speed: 0.0,
            colour: colour.unwrap_or(Srgb::from_color(Hsv::new(
                rng.gen_range(0.0..360.0),
                1.0,
                1.0,
            ))),
            direction: Direction::Up,
            last_update: Instant::now(),
            random_colour: colour.is_none(),
            gravity: gravity.unwrap_or(Self::DEFAULT_GRAVITY),
            bounciness: bounciness.unwrap_or(Self::DEFAULT_BOUNCINESS),
            speed_range: speed.unwrap_or(Self::DEFAULT_SPEEDS),
            current_bounciness: 0.0,
        }
    }

    fn reset(&mut self) {
        let mut rng = thread_rng();
        self.speed = rng.gen_range(self.speed_range.clone());
        self.colour = if self.random_colour {
            Srgb::from_color(Hsv::new(rng.gen_range(0.0..360.0), 1.0, 1.0))
        } else {
            self.colour
        };
        self.current_bounciness = rng.gen_range(self.bounciness.clone());
    }

    fn update(&mut self) {
        let mut elapsed = self.last_update.elapsed().as_secs_f32() / 1.0;
        if elapsed > 1.0 {
            self.reset();
            elapsed = 0.0;
        }
        self.last_update = Instant::now();

        match self.direction {
            Direction::Up => {
                let d1 = self.speed * elapsed / 2.0;
                self.speed -= self.gravity * elapsed;
                let d2 = self.speed * elapsed / 2.0;
                self.position += (d1 + d2).max(0.0);
                if self.speed <= 1.0 {
                    if self.position < 0.5 {
                        self.reset();
                    } else {
                        self.direction = Direction::Down;
                    }
                }
            }
            Direction::Down => {
                let d1 = self.speed * elapsed / 2.0;
                self.speed += self.gravity * elapsed;
                let d2 = self.speed * elapsed / 2.0;
                self.position -= (d1 + d2).max(0.0);
                if self.position <= 0.0 {
                    self.direction = Direction::Up;
                    self.speed *= self.current_bounciness;
                }
            }
        }
    }
    fn location(&self) -> usize {
        match self.direction {
            Direction::Up => self.position.floor() as usize,
            Direction::Down => self.position.ceil() as usize,
        }
    }
}

pub struct Bounce {
    count: usize,
    balls: Vec<Ball>,
}

impl Bounce {
    pub fn new(
        count: usize,
        colour: Option<Srgb>,
        balls: Option<usize>,
        gravity: Option<f32>,
        bounciness: Option<Range<f32>>,
        speed: Option<Range<f32>>,
    ) -> Self {
        let mut new_balls = Vec::new();
        for _ in 0..balls.unwrap_or(3) {
            new_balls.push(Ball::new(
                colour,
                gravity,
                bounciness.clone(),
                speed.clone(),
            ));
        }
        Bounce {
            count,
            balls: new_balls,
        }
    }
}

impl Effect for Bounce {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let mut out = vec![Srgb::<u8>::new(0, 0, 0); self.count];
        for ball in self.balls.iter_mut() {
            ball.update();
            let pixel = ball.location();
            let mut tail_len = (ball.speed * 0.5).ceil() as usize;
            if tail_len > pixel {
                tail_len = pixel;
            }
            for i in 0..tail_len as i32 {
                let pixel: i32 = match ball.direction {
                    Direction::Up => ball.location() as i32 - i,
                    Direction::Down => ball.location() as i32 + i,
                };
                if pixel < self.count as i32 && pixel >= 0 {
                    let mut colour: Srgb = ball.colour.into_format();
                    colour = colour.darken_fixed(i as f32 / tail_len as f32);
                    out[pixel as usize] = colour.into_format::<u8>();
                }
            }
            if pixel < self.count {
                out[pixel] = ball.colour.into_format::<u8>();
            }
        }
        Some(out)
    }
}
