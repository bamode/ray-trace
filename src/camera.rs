use rand::prelude::*;

use crate::math::ray::Ray;
use crate::render::{degrees_to_radians, Point};
use crate::vec::Vec3;

#[derive(Debug)]
pub struct Camera {
    origin: Point,
    ll_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    rng: ThreadRng,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        vfov: f64,
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let ll_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        let rng = thread_rng();

        Camera {
            origin,
            horizontal,
            vertical,
            ll_corner,
            u,
            v,
            lens_radius,
            rng,
        }
    }

    pub fn get_ray(&mut self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk(&mut self.rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.ll_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
