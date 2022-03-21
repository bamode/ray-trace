use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

use rand::prelude::*;

use crate::render::random_f64;

/// Ray tracers are concerned principally with calculating the geometry
/// of vectors in a three-dimensional space. Thus, it makes since to 
/// write a general purpose vector for R^3, and provide the usual 
/// utility functions to do vector math.
/// 
/// Ultimately, our vector is designed to provide a nice interface to what
/// ought be as fast as a three-tuple of `f64`. In addition to vector
/// math that is implemented on the `Vec3` struct itself, we also provide
/// implementations of all the usual operators. 
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1.0e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
    
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(self.y * other.z - self.z * other.y,
                  self.z * other.x - self.x * other.z,
                  self.x * other.y - self.y * other.x)
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn unit_vector(&self) -> Self { 
        *self / self.length()
    }

    #[inline]
    pub fn reflect(&self, n: Vec3) -> Self {
        *self - n * 2.0 * self.dot(&n)
    }

    #[inline]
    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = -self.dot(&n).min(1.0);
        let r_out_perp: Vec3 = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel: Vec3 = n * -1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn random(min: f64, max: f64, rng: &mut ThreadRng) -> Self {
        let r1 = random_f64(min, max, rng);
        let r2 = random_f64(min, max, rng);
        let r3 = random_f64(min, max, rng);
        Vec3::new(r1, r2, r3)
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::random(-1.0, 1.0, rng);
            if p.length_squared() >= 1.0 { continue }
            return p
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::random_in_unit_sphere(rng).unit_vector()
    }

    pub const X_HAT: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y_HAT: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z_HAT: Self = Self { x: 0.0, y: 0.0, z: 1.0 };
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) 
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) 
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{} {} {}]", self.x, self.y, self.z)?;
        
        Ok(())
    }
}
