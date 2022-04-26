use std::fs::File;
use std::io::{Result, Write};

use crate::hit::{Hit, HitList, HitRecord};
use crate::material::{MatKind, Material};
use crate::ray::Ray;
use crate::vec::Vec3;

use rand::prelude::*;

pub const PI: f64 = std::f64::consts::PI;
pub const DEG_TO_RAD: f64 = PI / 180.0;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * DEG_TO_RAD
}

#[inline]
pub fn random_f64(min: f64, max: f64, rng: &mut ThreadRng) -> f64 {
    rng.gen_range(min..max)
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }

    x
}

pub type Point = Vec3;

impl Point {
    pub const ORIGIN: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

pub type Color = Vec3;

pub fn write_color_to_pixel_buffer(color: Color, samples_per_pixel: usize) -> (u8, u8, u8) {
    let mut color = color;
    let scale = 1.0 / samples_per_pixel as f64;
    color.x = (scale * color.x).sqrt();
    color.y = (scale * color.y).sqrt();
    color.z = (scale * color.z).sqrt();

    let ir = (256.0 * clamp(color.x, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(color.y, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(color.z, 0.0, 0.999)) as u8;

    (ir, ig, ib)
}

pub fn write_buffer(file: &mut File, pixels: &[u8]) -> Result<()> {
    for pix in pixels.chunks(3).rev() {
        file.write_all(format!("{} {} {}\n", pix[0], pix[1], pix[2]).as_bytes())?;
    }
    Ok(())
}

pub fn ray_color(ray: &Ray, world: &HitList<MatKind>, depth: isize, rng: &mut ThreadRng) -> Color {
    let mut rec = HitRecord::empty();

    if depth <= 0 {
        //println!("[WARNING] depth limit reached");
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let mat = rec.material;
        let scatter = mat.scatter(ray, &rec, rng);
        if scatter.is_scattered {
            return ray_color(&scatter.scattered, world, depth - 1, rng) * scatter.attenuation;
        }
        return Color::default();
    }

    let unit_dir = ray.dir.unit_vector();
    let t = (unit_dir.y + 1.0) * 0.5;

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
