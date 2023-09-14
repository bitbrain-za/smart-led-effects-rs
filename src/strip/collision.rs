use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, Mix, Srgb};
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
            size: 1,
        }
    }

    pub fn collide(&self, other: &Particle) -> Option<(Particle, Particle)> {
        if self.position != other.position {
            return None;
        }
        let mut rhs = other.clone();
        let mut lhs = self.clone();
        lhs.reverse = !lhs.reverse;
        rhs.reverse = !rhs.reverse;

        let scaling_factor = lhs.size as f32 / (lhs.size + rhs.size) as f32;
        let mix = lhs.colour.mix(rhs.colour, scaling_factor);
        lhs.colour = mix;
        rhs.colour = mix;

        Some((lhs, rhs))
    }
}

pub struct Collision {
    particles: Vec<Particle>,
    count: usize,
}

impl Collision {
    pub fn new(count: usize) -> Self {
        let p1 = Particle::new(0, false);
        let p2 = Particle::new(count as i32, true);

        Collision {
            count,
            particles: vec![p1, p2],
        }
    }

    pub fn reset(&mut self) {
        let p1 = Particle::new(0, false);
        let p2 = Particle::new(self.count as i32 - 1, true);

        self.particles = vec![p1, p2];
    }

    pub fn check_for_collision(&mut self) {
        if let Some((lhs, rhs)) = self.particles[0].collide(&self.particles[1]) {
            self.particles[0] = lhs;
            self.particles[1] = rhs;
        }
    }
}

impl EffectIterator for Collision {
    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        let mut out = vec![Srgb::<u8>::new(0, 0, 0); self.count];

        for particle in self.particles.iter_mut() {
            if particle.reverse {
                particle.position -= particle.speed as i32;
            } else {
                particle.position += particle.speed as i32;
            }
        }
        if self.particles[0].position < 0 && self.particles[1].position >= self.count as i32 {
            self.reset();
        }
        for particle in self.particles.iter_mut() {
            if particle.position >= 0 && particle.position < self.count as i32 {
                out[particle.position as usize] = particle.colour.into_format();
            }
        }

        Some(out)
    }
}
