use std::fs::File;
use std::io::{Result, Write};

use crate::vec::Vec3;
use crate::ray::Ray;

pub type Point = Vec3;
pub type Color = Vec3;

pub fn write_color(file: &mut File, color: Color) -> Result<()> {
    let ir = (color.x * 256.999).round() as u8;
    let ig = (color.y * 256.999).round() as u8;
    let ib = (color.z * 256.999).round() as u8;

    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    Ok(())
}

pub fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
    }
    
    let unit_dir = ray.dir.unit_vector();
    let t: f64 = 0.5 * (unit_dir.y + 1.0);
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
