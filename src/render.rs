use std::fs::File;
use std::io::{Result, Write};

use crate::hit::{Hit, HitList, HitRecord};
use crate::material::{Material, MatKind};
use crate::vec::Vec3;
use crate::ray::Ray;

use rand::prelude::*;

const PI: f64 = 3.141592653589793285;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees *  PI / 180.0
}

#[inline]
pub fn random_f64(min: f64, max: f64, rng: &mut ThreadRng) -> f64 {
    min + (max - min) * rng.gen::<f64>()
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min }
    else if x > max { return max }

    x
}

pub type Point = Vec3;

impl Point {
    pub const ORIGIN: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
}

pub type Color = Vec3;

pub fn write_color(file: &mut File, color: Color, samples_per_pixel: usize) -> Result<()> {
    let mut color = color;
    let scale = 1.0 / samples_per_pixel as f64;
    color.x = (scale * color.x).sqrt();
    color.y = (scale * color.y).sqrt();
    color.z = (scale * color.z).sqrt();

    let ir = (256.0 * clamp(color.x, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(color.y, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(color.z, 0.0, 0.999)) as u8;

    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    Ok(())
}

pub fn ray_color(ray: &Ray, world: &HitList<MatKind>, depth: isize, rng: &mut ThreadRng) -> Color {
    let mut rec = HitRecord::empty();

    if depth <= 0 { 
        //println!("[WARNING] depth limit reached");
        return Color::new(0.0, 0.0, 0.0) 
    }

    if world.hit(ray, 0.0001, f64::INFINITY, &mut rec) {
        let mat = rec.material;
        let scatter = mat.scatter(&ray, &mut rec, rng);
        if scatter.is_scattered {
            return ray_color(&scatter.scattered, world, depth - 1, rng) * scatter.attenuation
        }
        return Color::default()
    }

    let unit_dir = ray.dir.unit_vector();
    let t = (unit_dir.y + 1.0) * 0.5;
    
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
