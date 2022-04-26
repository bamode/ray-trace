#![allow(unused)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use rand::prelude::*;

use super::Dot;
use super::vec::Vec3;
use super::super::render::random_f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normal {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Normal {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Normal { x, y, z }
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
        Self::new(self.y * other.z - self.z * other.y,
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

    #[allow(unused)]
    #[inline]
    pub fn coordinate_system_from(&self) -> (Self, Self, Self) {
        let n1 = self.unit_vector();

        let n2 = if n1.x.abs() > n1.y.abs() {
            let norm_len = (n1.x * n1.x + n1.z * n1.z).sqrt();
            Self::new(-n1.z, 0., n1.x) / norm_len
        } else {
            let norm_len = (n1.y * n1.y + n1.z * n1.z).sqrt();
            Self::new(0., n1.z, -n1.y) / norm_len
        };

        let n3 = n1.cross(&n2);
        
        (n1, n2, n3)
    }

    #[allow(unused)]
    #[inline]
    pub fn min_component(&self) -> f64 {
        self.x.min(self.y.min(self.z))
    }

    #[allow(unused)]
    #[inline]
    pub fn max_component(&self) -> f64 {
        self.x.max(self.y.max(self.z))
    }

    #[allow(unused)]
    #[inline]
    pub fn min(&self, other: &Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y), self.z.min(other.z))
    }

    #[allow(unused)]
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y), self.z.max(other.z))
    }

    /// Unimplemented because I want to avoid this sort of thing.
    /// That said, it could be doable with pattern matching.
    #[allow(unused)]
    pub fn max_dimension(&self) -> usize {
        unimplemented!()
    }

    /// Unimplemented because I don't quite understand what the idea is. I'm 
    /// concerned that it's very unsafe, so I want to see it in action before
    /// making it available as a tool.
    #[allow(unused)]
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn reflect(&self, n: Self) -> Self {
        *self - n * 2.0 * self.dot(&n)
    }

    #[inline]
    pub fn refract(&self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = -self.dot(&n).min(1.0);
        let r_out_perp: Self = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel: Self = n * -1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn random(min: f64, max: f64, rng: &mut ThreadRng) -> Self {
        let r1 = random_f64(min, max, rng);
        let r2 = random_f64(min, max, rng);
        let r3 = random_f64(min, max, rng);
        Self::new(r1, r2, r3)
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::random(-1.0, 1.0, rng);
            if p.length_squared() >= 1.0 { continue }
            return p
        }
    }

    pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::new(random_f64(-1.0, 1.0, rng), random_f64(-1.0, 1.0, rng), 0.0);
            if p.length_squared() < 1.0 { return p }
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::random_in_unit_sphere(rng).unit_vector()
    }
}

impl From<Vec3> for Normal {
    fn from(v: Vec3) -> Normal {
        Normal::new(v.x, v.y, v.z)
    }
}

impl Dot<Normal> for Normal {
    #[inline(always)]
    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Dot<Normal> for Vec3 {
    #[inline(always)]
    fn dot(&self, rhs: &Normal) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Dot<Vec3> for Normal {
    #[inline(always)]
    fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add<Normal> for Normal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Normal {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Normal> for Normal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Normal {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Normal {
    type Output = Normal;

    fn mul(self, rhs: f64) -> Normal {
        Normal::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Normal> for f64 {
    type Output = Normal;

    fn mul(self, rhs: Normal) -> Normal {
        Normal::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl MulAssign<f64> for Normal {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Normal {
    type Output = Normal;

    fn div(self, rhs: f64) -> Normal {
        Normal::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Normal {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}