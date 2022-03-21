use std::fs::File;
use std::io::{Result, Write};

mod camera;
mod hit;
mod material;
mod ray;
mod render;
mod sphere;
mod vec;

use crate::camera::Camera;
use crate::hit::{HitList, Hittable};
use crate::material::{Dielectric, Lambertian, Metal, MatKind};
use crate::render::{Color, Point, write_color, ray_color};
use crate::sphere::Sphere;
use crate::vec::Vec3;

use progress::Bar;
use rand::prelude::*;

fn main() -> Result<()> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 1900;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: isize = 100;

    // World
    let mut world: HitList<MatKind> = HitList::new();

    let ground = MatKind::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = MatKind::Lambertian(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left = MatKind::Dielectric(Dielectric::new(1.5));
    let right = MatKind::Metal(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.push(Hittable::Sphere(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, center)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.45, left)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right)));

    // Camera
    let camera = Camera::new(ASPECT_RATIO, 20.0, Point::new(-2.0, 2.0, 1.0), Point::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0)); 

    // Render 
    let mut file = File::create("image.ppm").unwrap();
    
    file.write_all("P3\n".as_bytes())?;
    file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    let mut bar = Bar::new();
    bar.set_job_title("Rendering...");

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>())/ (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH, &mut rng);
            }
            write_color(&mut file, pixel_color, SAMPLES_PER_PIXEL)?;

            let prog: i32 = (((IMAGE_HEIGHT - j) * IMAGE_WIDTH + i) as f64 / (IMAGE_WIDTH as f64 * IMAGE_HEIGHT as f64) * 100.0) as i32;
            bar.reach_percent(prog);
        }
    }

    Ok(())
}
