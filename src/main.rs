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
use crate::render::{Color, Point, ray_color};
use crate::sphere::Sphere;
use crate::vec::Vec3;

// haven't figured out how to use this with rayon yet, maybe I could just track one thread as a proxy?
use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rayon::prelude::*;

fn main() -> Result<()> {
    // RNG
    // The camera has to live for as long as rendering takes, and we need random numbers
    // both for some camera functionality and for processing the ray after the camera
    // spits one out. Thus, we need to have two separate threads for the RNG. We take the
    // approach of passing around mutable references to `ThreadRng` structs from the `rand`
    // crate as they claim that this is a bit faster than spawning a new `ThreadRng` process
    // implicitly with calls to `random` since we need random numbers quite often. Here, we
    // make an additional `ThreadRng` for randomly generating the scene.
    let mut world_rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: isize = 50;

    // World
    let world = random_scene(&mut world_rng);

    // Camera
    const VFOV: f64 = 20.0;
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;


    // Render 
    let mut file = File::create("image.ppm").unwrap();
    
    file.write_all("P3\n".as_bytes())?;
    file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    // trying to rewrite in parallel
    let mut pixels = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT * 3];

    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(IMAGE_WIDTH * 3).enumerate().collect();
    bands.into_par_iter().progress_count(IMAGE_HEIGHT as u64).for_each(|(j, band)| {
        let mut rng = rand::thread_rng();
        let mut cam_rng = rand::thread_rng();
        // Only needs to be mutable for the RNG to work.
        let mut camera = Camera::new(ASPECT_RATIO, VFOV, lookfrom, lookat, vup, aperture, dist_to_focus, &mut cam_rng); 
        for i in (0..IMAGE_WIDTH).rev() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH, &mut rng);
            }
            let pix_result = render::write_color_to_pixel_buffer(pixel_color, SAMPLES_PER_PIXEL);
            band[i * 3] = pix_result.0;
            band[i * 3 + 1] = pix_result.1;
            band[i * 3 + 2] = pix_result.2;
        }
    });

    render::write_buffer(&mut file, &pixels)?;

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
