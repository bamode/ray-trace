use crate::ray::Ray;
use crate::render::{degrees_to_radians, Point};
use crate::vec::Vec3;

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    origin: Point,
    ll_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, vfov: f64, lookfrom: Point, lookat: Point, vup: Vec3) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let ll_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera { 
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            ll_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, self.ll_corner + self.horizontal * s + self.vertical * t - self.origin)
    }
}