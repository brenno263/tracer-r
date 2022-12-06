use rand::prelude::*;
use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::traits::WeightedMean;

const EPLISON: f32 = 0.0001;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }

    pub fn zero() -> V3 {
        V3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> V3 {
        V3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn x() -> V3 {
        V3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn y() -> V3 {
        V3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }
    pub fn z() -> V3 {
        V3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> V3 {
        V3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(self) -> V3 {
        let inverse_length = 1.0 / self.magnitude();
        self * inverse_length
    }

    pub fn scalar_projection(&self, other: &V3) -> f32 {
        self.dot(&other.normalized())
    }

    pub fn projection(&self, other: &V3) -> V3 {
        other.normalized() * self.scalar_projection(other)
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPLISON && self.y.abs() < EPLISON && self.z.abs() < EPLISON
    }

    pub fn random() -> V3 {
        let mut rand = rand::thread_rng();
        V3 {
            x: rand.gen(),
            y: rand.gen(),
            z: rand.gen(),
        }
    }

    pub fn random_in_range(min: f32, max: f32) -> V3 {
        assert!(min >= 0.0 && max >= 0.0);
        let delta = max - min;
        let mut rand = rand::thread_rng();
        V3 {
            x: rand.gen::<f32>() * delta + min,
            y: rand.gen::<f32>() * delta + min,
            z: rand.gen::<f32>() * delta + min,
        }
    }

    pub fn random_in_unit_sphere() -> V3 {
        let mut attempt = V3::random();
        while attempt.dot(&attempt) > 1.0 {
            attempt = V3::random();
        }
        attempt
    }

    pub fn random_on_unit_sphere() -> V3 {
        V3::random().normalized()
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> Self::Output {
        V3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for V3 {
    fn sum<I: Iterator<Item = V3>>(iter: I) -> Self {
        iter.fold(V3::zero(), |acc, v| acc + v)
    }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, rhs: Self) -> Self::Output {
        V3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for V3 {
    type Output = V3;

    fn mul(self, rhs: f32) -> Self::Output {
        V3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for V3 {
    type Output = V3;

    fn div(self, rhs: f32) -> Self::Output {
        let s = 1.0 / rhs;
        V3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl std::fmt::Display for V3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub point: V3,
    pub normal: V3,
}

impl<T> WeightedMean for T
where
    T: AddAssign + Mul<f64, Output = T> + Div<f64, Output = T> + Copy + Default,
{
    fn weighted_mean(it: impl Iterator<Item = (T, f64)>) -> Option<T> {
        let (sum, total_weight) = it.fold(
            (T::default(), 0.0),
            |(mut sum, total_weight), (value, weight)| {
                sum += value * weight;
                (sum, total_weight + weight)
            },
        );
        if total_weight.is_normal() {
            Some(sum / total_weight)
        } else {
            None
        }
    }
}
