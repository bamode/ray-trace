use std::fs::File;
use std::io::{Result, Write};

mod camera;
mod hit;
mod material;
mod ray;
mod render;
mod sphere;
mod math;

use crate::camera::Camera;
use crate::hit::{HitList, Hittable};
use crate::material::{Dielectric, Lambertian, Metal, MatKind};
use crate::render::{Color, Point, ray_color};
use crate::sphere::Sphere;
use crate::math::*;

use clap::Parser;
use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Parser)]
#[clap(name = "ray-trace",
       author = "Brent Mode <bmode@wisc.edu>",
       version,
       about = "Parallelized MC ray tracing renderer written in Rust",
       long_about = "For now, this is a demo that only creates one semi-random scene")]
struct Cli {
    #[clap(short, long)]
    out: Option<String>,
    #[clap(short, long)]
    width: Option<usize>,
    #[clap(short = 'i', long)]
    height: Option<usize>,
    #[clap(short, long)]
    samples: Option<usize>,
    #[clap(short, long)]
    depth: Option<isize>,
}

/// This project is in following with Peter Shirley's excellent Ray Tracing in a Weekend book. 
fn main() -> Result<()> {
    // CLI
    let cli = Cli::parse();

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
    let width = cli.width.unwrap_or(400);
    let height = cli.height.unwrap_or((width as f64 * 9.0 / 16.0) as usize);
    let samples = cli.samples.unwrap_or(100);
    let depth = cli.depth.unwrap_or(32);
    let aspect_ratio = width as f64 / height as f64;

    // World
    let world = random_scene(&mut world_rng);

    // Camera
    // TODO: It would be neat to be able to specify these in someway to describe a series
    // of different scenes. I think this is beyond a tasteful CLI though, so it'll have 
    // to wait on me writing a TOML or JSON scene descripter with serde.
    const VFOV: f64 = 20.0;
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;


    // Render 
    let filename = cli.out.unwrap_or("image.ppm".to_string());
    let mut file = File::create(filename).unwrap();
    
    file.write_all("P3\n".as_bytes())?;
    file.write_all(format!("{} {}\n", width, height).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    // trying to rewrite in parallel
    let mut pixels = vec![0; width * height * 3];

    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(width * 3).enumerate().collect();
    bands.into_par_iter().progress_count(height as u64).for_each(|(j, band)| {
        let mut rng = rand::thread_rng();
        // Only needs to be mutable for the RNG to work.
        let mut camera = Camera::new(aspect_ratio, VFOV, lookfrom, lookat, vup, aperture, dist_to_focus); 
        for i in (0..width).rev() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples {
                let u = (i as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, depth, &mut rng);
            }
            let pix_result = render::write_color_to_pixel_buffer(pixel_color, samples);
            band[i * 3] = pix_result.0;
            band[i * 3 + 1] = pix_result.1;
            band[i * 3 + 2] = pix_result.2;
        }
    });

    render::write_buffer(&mut file, &pixels)?;

    Ok(())
}

/// This generates a random scene using the same business logic as for the scene on the cover of the book.
/// We make several different small spheres, somewhat randomly positioning them and assigning them a material.
fn random_scene(rng: &mut ThreadRng) -> HitList<MatKind> {
    let mut world: HitList<MatKind> = HitList::new();
    
    let ground = MatKind::Lambertian(Lambertian::new(Color::new(0.5117, 0.2539, 0.0977)));
    world.push(Hittable::Sphere(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mat: MatKind;
                if choose_mat < 0.6 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0, rng) * Color::random(0.0, 1.0, rng);
                    sphere_mat = MatKind::Lambertian(Lambertian::new(albedo));
                    world.push(Hittable::Sphere(Sphere::new(center, 0.2, sphere_mat)));
                } else if choose_mat < 0.8 {
                    // metal
                    let albedo = Color::random(0.2, 1.0, rng);
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
