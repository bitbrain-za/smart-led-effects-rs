use palette::{Darken, FromColor, Hsv, Srgb};
use rand::{thread_rng, Rng};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Ball {
    position: f32,
    speed: f32,
    colour: Srgb,
    direction: Direction,
    last_update: Instant,
    pixels_per_mm: f32,
    random: bool,
}

impl Ball {
    const GRAVITY: f32 = 9.8;
    const BOUNCINESS: f32 = 0.9;
    fn new(colour: Option<Srgb>, pixels_per_mm: f32) -> Self {
        let mut rng = thread_rng();
        Ball {
            position: 0.0,
            speed: rng.gen_range(2.3..5.0),
            colour: colour.unwrap_or(Srgb::from_color(Hsv::new(
                rng.gen_range(0.0..360.0),
                1.0,
                1.0,
            ))),
            direction: Direction::Up,
            last_update: Instant::now(),
            pixels_per_mm,
            random: colour.is_none(),
        }
    }

    fn update(&mut self) {
        let elapsed = self.last_update.elapsed().as_secs_f32() / 8.0;
        self.last_update = Instant::now();

        match self.direction {
            Direction::Up => {
                let d1 = self.speed * elapsed / 2.0;
                self.speed -= Self::GRAVITY * elapsed;
                let d2 = self.speed * elapsed / 2.0;
                self.position += (d1 + d2).max(0.0);
                if self.speed <= 0.0 {
                    self.direction = Direction::Down;
                    if self.position < 0.01 {
                        let mut rng = thread_rng();
                        self.speed = rng.gen_range(2.0..5.0);
                        self.colour = if self.random {
                            Srgb::from_color(Hsv::new(rng.gen_range(0.0..360.0), 1.0, 1.0))
                        } else {
                            self.colour
                        };
                    }
                }
            }
            Direction::Down => {
                let d1 = self.speed * elapsed / 2.0;
                self.speed += Self::GRAVITY * elapsed;
                let d2 = self.speed * elapsed / 2.0;
                self.position -= (d1 + d2).max(0.0);
                if self.position <= 0.0 {
                    self.direction = Direction::Up;
                    self.speed *= Self::BOUNCINESS;
                }
            }
        }
    }
    fn location(&self) -> usize {
        (self.position * 1000.0 * self.pixels_per_mm).floor() as usize
    }
}

pub struct Bounce {
    count: usize,
    balls: Vec<Ball>,
}

impl Bounce {
    pub fn new(
        count: usize,
        spacing_mm: Option<f32>,
        colour: Option<Srgb>,
        balls: Option<usize>,
    ) -> Self {
        let mut new_balls = Vec::new();
        for _ in 0..balls.unwrap_or(2) {
            new_balls.push(Ball::new(colour, 1.0 / spacing_mm.unwrap_or(16.0)));
        }
        Bounce {
            count,
            balls: new_balls,
        }
    }
}

impl Iterator for Bounce {
    type Item = Vec<Srgb<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = vec![Srgb::<u8>::new(0, 0, 0); self.count];
        for ball in self.balls.iter_mut() {
            ball.update();
            let pixel = ball.location();
            if pixel < self.count {
                out[pixel] = ball.colour.into_format::<u8>();
            }

            let tail_len = (ball.speed * 8.0).ceil() as usize;
            for i in 0..tail_len as i32 {
                let pixel: i32 = match ball.direction {
                    Direction::Up => ball.location() as i32 + i,
                    Direction::Down => ball.location() as i32 - i,
                };
                if pixel < self.count as i32 && pixel >= 0 {
                    let mut colour: Srgb = ball.colour.into_format();
                    colour = colour.darken_fixed(1.0 - (i as f32 / tail_len as f32));
                    out[pixel as usize] = colour.into_format::<u8>();
                }
            }
        }
        Some(out)
    }
}
