use std::fs::File;
use std::io::{Result, Write};

use crate::hit::{Hit, HitList, HitRecord};
use crate::vec::Vec3;
use crate::ray::Ray;

const PI: f64 = 3.141592653589793285;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees *  PI / 180.0
}

pub type Point = Vec3;

impl Point {
    pub const ORIGIN: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
}

pub type Color = Vec3;

pub fn write_color(file: &mut File, color: Color) -> Result<()> {
    let ir = (color.x * 256.999).round() as u8;
    let ig = (color.y * 256.999).round() as u8;
    let ib = (color.z * 256.999).round() as u8;

    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    Ok(())
}

pub fn ray_color(ray: &Ray, world: &HitList) -> Color {
    let mut rec = HitRecord::empty();
    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
    }
    let unit_dir = ray.dir.unit_vector();
    let t = (unit_dir.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

#[inline]
pub fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.dir.length_squared();
    let b = 2.0 * oc.dot(&ray.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
