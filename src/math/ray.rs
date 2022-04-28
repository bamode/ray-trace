#![allow(unused)]

use super::point::Point3;
use super::vec::Vec3;
use crate::render::Point;

/// So, in PBRT, they use this C++ construction that gives them otherwise
/// immutable (`const` in C++ parlance) `Ray` instances with a mutable field to
/// represent the upper bound `t` for the ray.
///
/// A ray is defined mathematically as r(t) = o + td, 0 < t < Infinity where
/// r is a scalar function of a parameter t that is equal to an origin point o and
/// a direction unit vector d with length t. The idea of this mutable field `t_max`
/// is to restrict the ray to the bounds [0, r(t_max)]. I'm not really sure how to
/// do this in Rust, so I'm planning to wait until I use it to figure out a convenient
/// implementation.
///
/// TODO: This should use `Point3` instead of `Point`.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Point,
    pub t_max: f64,
    pub time: f64,
    pub medium: Medium,
}

impl Ray {
    pub fn new(origin: Point, dir: Point) -> Self {
        Self {
            origin,
            dir,
            t_max: std::f64::INFINITY,
            time: 0.0,
            medium: Medium,
        }
    }

    #[inline(always)]
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Medium;

#[derive(Clone, Copy, Debug)]
pub struct RayDifferential {
    ray: Ray,
    has_differentials: bool,
    rx_origin: Point3,
    ry_origin: Point3,
    rx_dir: Vec3,
    ry_dir: Vec3,
}

impl RayDifferential {
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            has_differentials: false,
            rx_origin: Point3::new(0., 0., 0.),
            ry_origin: Point3::new(0., 0., 0.),
            rx_dir: Vec3::new(0., 0., 0.),
            ry_dir: Vec3::new(0., 0., 0.),
        }
    }

    pub fn scale_differentials(&mut self, s: f64) {
        self.rx_origin = self.ray.origin + (self.rx_origin - self.ray.origin) * s;
    }
}
