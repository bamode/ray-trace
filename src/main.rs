use std::fs::File;
use std::io::{Result, Write};

mod hit;
mod ray;
mod render;
mod sphere;
mod vec;

use crate::vec::Vec3;
use crate::ray::Ray;
use crate::render::{Color, Point, write_color, ray_color};

use progress::Bar;

fn main() -> Result<()> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 1080;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    

    // Render 
    let mut file = File::create("image.ppm").unwrap();
    
    file.write_all("P3\n".as_bytes())?;
    file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    let mut bar = Bar::new();
    bar.set_job_title("Rendering...");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let color: Color = ray_color(&ray);
            write_color(&mut file, color)?;

            let prog: i32 = (((IMAGE_HEIGHT - j) * IMAGE_WIDTH + i) as f64 / (IMAGE_WIDTH as f64 * IMAGE_HEIGHT as f64) * 100.0) as i32;
            bar.reach_percent(prog);
        }
    }

    Ok(())
}
