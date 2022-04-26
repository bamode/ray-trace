#![allow(unused)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

use super::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Point3> for Point2 {
    fn from(p: Point3) -> Point2 {
        Point2::new(p.x, p.y)
    }
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn distance(self, p: Self) -> f64 {
        (self - p).length()
    }

    #[inline]
    pub fn distance_squared(self, p: Self) -> f64 {
        (self - p).length_squared()
    }

    #[inline]
    pub fn linterp(t: f64, p1: Self, p2: Self) -> Self {
        (1.0 - t) * p1 + t * p2
    }

    #[inline]
    pub fn min(self, p: Self) -> Self {
        Point3::new(self.x.min(p.x), self.y.min(p.y), self.z.min(p.z))
    }

    #[inline]
    pub fn max(self, p: Self) -> Self {
        Point3::new(self.x.max(p.x), self.y.max(p.y), self.z.max(p.z))
    }

    #[inline]
    pub fn floor(self) -> Self {
        Point3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    #[inline]
    pub fn ceil(self) -> Self {
        Point3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    #[inline]
    pub fn abs(self) -> Self {
        Point3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Much like in `crate::math::vec::Vec3`, I don't really think this is a safe,
    /// good function to use, but I don't know how it's going to crop up, so I'm not
    /// sure how feasible avoiding it actually is in practice. So I'll put it in here,
    /// leave it unimplemented, and then bam, reminder when I need it. Because it won't
    /// work. The compiler'll just shit itself.
    fn permute() {
        unimplemented!()
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Point3) -> Point3 {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vec3) -> Point3 {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Point3> for Vec3 { 
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign<Point3> for Point3 {
    fn add_assign(&mut self, other: Point3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, other: Point3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vec3) -> Point3 {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Point3> for Vec3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign<Point3> for Point3 {
    fn sub_assign(&mut self, other: Point3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<Vec3> for Point3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, other: f64) -> Point3 {
        Point3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        Point3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl MulAssign<f64> for Point3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}