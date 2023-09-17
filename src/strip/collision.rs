use crate::strip::EffectIterator;
use palette::{Darken, FromColor, Hsv, Mix, Srgb};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    position: i32,
    colour: Srgb,
    reverse: bool,
    speed: usize,
    size: usize,
}

impl Particle {
    pub fn new(position: i32, reverse: bool) -> Self {
        let mut rng = thread_rng();

        Particle {
            position,
            colour: Srgb::from_color(Hsv::new(rng.gen_range(0.0..360.0), 1.0, 1.0)),
            reverse,
            speed: 1,
            size: rng.gen_range(1..4),
        }
    }

    pub fn collide(&self, other: &Particle) -> Option<(Particle, Particle)> {
        if (self.position - other.position).abs() > 1 || self.position - other.position < -1 {
            return None;
        }
        let mut rhs = *other;
        let mut lhs = *self;
        lhs.reverse = !lhs.reverse;
        rhs.reverse = !rhs.reverse;

        let scaling_factor = 1.0 - lhs.size as f32 / (lhs.size + rhs.size) as f32;
        let mix = lhs.colour.mix(rhs.colour, scaling_factor / 2.0);
        lhs.colour = mix;
        rhs.colour = mix;

        Some((lhs, rhs))
    }
}

pub struct Collision {
    particles: Vec<Particle>,
    count: usize,
    shatter: bool,
    shattered: bool,
    current: Vec<Srgb>,
}

impl Collision {
    pub fn new(count: usize, shatter: Option<bool>) -> Self {
        let p1 = Particle::new(0, false);
        let p2 = Particle::new(count as i32, true);

        Collision {
            count,
            particles: vec![p1, p2],
            shatter: shatter.unwrap_or(true),
            shattered: false,
            current: vec![Srgb::new(0.0, 0.0, 0.0); count],
        }
    }

    pub fn reset(&mut self) {
        let p1 = Particle::new(0, false);
        let p2 = Particle::new(self.count as i32 - 1, true);

        self.particles = vec![p1, p2];
        self.shattered = false;
    }

    pub fn check_for_collision(&mut self) -> bool {
        if let Some((lhs, rhs)) = self.particles[0].collide(&self.particles[1]) {
            self.particles[0] = lhs;
            self.particles[1] = rhs;

            true
        } else {
            false
        }
    }

    pub fn shatter(&mut self) {
        if !self.shatter {
            return;
        }
        self.shattered = true;

        self.current[self.count / 2] = Srgb::new(1.0, 1.0, 1.0);

        let mut hsv = Hsv::from_color(self.particles[0].colour);
        hsv.value = 1.0;
        let normalize = 1.0 / self.count as f32;

        let mut rng = thread_rng();

        for i in 0..(self.count / 2) {
            if rng.gen_range(0.0..1.0) < 0.5 {
                let hsv = hsv.darken(1.0 - normalize * i as f32);
                self.current[i] = Srgb::from_color(hsv);
            }
        }
        for i in (self.count / 2)..(self.count) {
            if rng.gen_range(0.0..1.0) < 0.5 {
                let hsv = hsv.darken(normalize * i as f32);
                self.current[i] = Srgb::from_color(hsv);
            }
        }
    }

    pub fn move_particles(&mut self) -> Vec<Srgb<u8>> {
        let mut out = vec![Srgb::<u8>::new(0, 0, 0); self.count];
        for particle in self.particles.iter_mut() {
            if particle.position >= 0 && particle.position < self.count as i32 {
                for i in 0..particle.size {
                    if particle.reverse {
                        if particle.position + i as i32 >= 0
                            && i as i32 + particle.position < self.count as i32
                        {
                            out[(particle.position + i as i32) as usize] =
                                particle.colour.into_format();
                        }
                    } else if particle.position - i as i32 >= 0
                        && (particle.position - i as i32) < self.count as i32
                    {
                        out[(particle.position - i as i32) as usize] =
                            particle.colour.into_format();
                    }
                }
                out[particle.position as usize] = particle.colour.into_format()
            }
        }
        out
    }
}

impl EffectIterator for Collision {
    fn name(&self) -> &'static str {
        "Collision"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        if !self.shattered {
            for particle in self.particles.iter_mut() {
                if particle.reverse {
                    particle.position -= particle.speed as i32;
                } else {
                    particle.position += particle.speed as i32;
                }
            }

            if self.check_for_collision() {
                self.shatter();
            }

            if self.particles[0].position < 0 && self.particles[1].position >= self.count as i32 {
                self.reset();
            }

            Some(self.move_particles())
        } else {
            let mut rng = thread_rng();
            for pixel in self.current.iter_mut() {
                if rng.gen_range(0.0..1.0) < 0.5 {
                    *pixel = pixel.darken(0.1);
                }
            }

            const RESET_VAL: f32 = 0.01;

            for pixel in self.current.iter() {
                if pixel.red > RESET_VAL || pixel.green > RESET_VAL || pixel.blue > RESET_VAL {
                    return Some(self.current.iter().map(|x| x.into_format()).collect());
                }
            }
            self.reset();
            Some(vec![Srgb::<u8>::new(0, 0, 0); self.count])
        }
    }
}
