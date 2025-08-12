
use dioxus_motion::{animations::utils::Animatable, prelude::Spring};


pub fn make_spring(stiffness: f32, mass: f32, damping_ratio: f32) -> Spring {
    let damping = 2.0 * damping_ratio * f32::sqrt(mass * stiffness);
    Spring { stiffness: stiffness, damping, mass, velocity: 0.0 }
}

#[derive(Debug, Clone, Copy)]
pub struct Animate4D(pub f32, pub f32, pub f32, pub f32);

impl Animatable for Animate4D {
    fn zero() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }

    fn epsilon() -> f32 {
        0.01
    }

    fn magnitude(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3).sqrt()
    }

    fn scale(&self, factor: f32) -> Self {
        Self(self.0 * factor, self.1 * factor, self.2 * factor, self.3 * factor)
    }

    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }

    fn sub(&self, other: &Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3)
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        Self(
            self.0 + (self.0 - target.0) * t, 
            self.1 + (self.1 - target.1) * t, 
            self.2 + (self.2 - target.2) * t,
            self.3 + (self.3 - target.3) * t,
        )
    }
}