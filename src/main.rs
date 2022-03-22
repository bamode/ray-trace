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
    // RNG
    let mut cam_rng = rand::thread_rng();
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: isize = 32;

    // World
    let world = random_scene(&mut rng);

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    // Only needs to be mutable for the RNG to work.
    let mut camera = Camera::new(ASPECT_RATIO, 20.0, lookfrom, lookat, vup, aperture, dist_to_focus, &mut cam_rng); 

    // Render 
    let mut file = File::create("image.ppm").unwrap();
    
    file.write_all("P3\n".as_bytes())?;
    file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    let mut bar = Bar::new();
    bar.set_job_title("Rendering...");

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

fn random_scene(rng: &mut ThreadRng) -> HitList<MatKind> {
    let mut world: HitList<MatKind> = HitList::new();
    
    let ground = MatKind::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mat: MatKind;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0, rng) * Color::random(0.0, 1.0, rng);
                    sphere_mat = MatKind::Lambertian(Lambertian::new(albedo));
                    world.push(Hittable::Sphere(Sphere::new(center, 0.2, sphere_mat)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0, rng);
                    sphere_mat = MatKind::Metal(Metal::new(albedo));
                    world.push(Hittable::Sphere(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    // glass
                    sphere_mat = MatKind::Dielectric(Dielectric::new(1.5));
                    world.push(Hittable::Sphere(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = MatKind::Dielectric(Dielectric::new(1.5));
    world.push(Hittable::Sphere(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = MatKind::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = MatKind::Metal(Metal::new(Color::new(0.7, 0.6, 0.5)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}
