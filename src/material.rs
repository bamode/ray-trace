use rand::prelude::*;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::render::Color;
use crate::vec::Vec3;

pub struct Scatter {
    pub is_scattered: bool,
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self,
               r_in: &Ray, 
               hit_record: &HitRecord<MatKind>, 
               rng: &mut ThreadRng) -> Scatter;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, 
               _r_in: &Ray, 
               hit_record: &HitRecord<MatKind>, 
               rng: &mut ThreadRng) -> Scatter 
    {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);

        if scatter_direction.near_zero() { scatter_direction = hit_record.normal; }
        
        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        let is_scattered = true;
        
        Scatter { is_scattered, attenuation, scattered }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, 
               r_in: &Ray, 
               hit_record: &HitRecord<MatKind>, 
               _rng: &mut ThreadRng) -> Scatter
    {
        let reflected = r_in.dir.unit_vector().reflect(hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        let attenuation = self.albedo;
        let is_scattered = scattered.dir.dot(&hit_record.normal) > 0.0;
        
        Scatter { is_scattered, attenuation, scattered }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    pub fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_sq = r0 * r0;
        r0_sq + (1.0 - r0_sq) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self,
               r_in: &Ray,
               hit_record: &HitRecord<MatKind>,
               rng: &mut ThreadRng) -> Scatter
    {
        let attenuation = Color::new(0.98, 0.98, 0.98);
        let refraction_ratio = 
            if hit_record.front_face.unwrap() { 1.0 / self.ir }
            else { self.ir };

        let unit_dir = r_in.dir.unit_vector();
        let cos_theta: f64 = -unit_dir.dot(&hit_record.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let dir = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() { unit_dir.reflect(hit_record.normal) }
            else { unit_dir.refract(hit_record.normal, refraction_ratio) };

        let scattered = Ray::new(hit_record.p, dir);
        let is_scattered = true;

        Scatter { is_scattered, attenuation, scattered }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MatKind {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for MatKind {
    fn scatter(&self, 
               r_in: &Ray, 
               hit_record: &HitRecord<MatKind>, 
               rng: &mut ThreadRng) -> Scatter
    {
        match self {
            Self::Lambertian(l) => l.scatter(r_in, hit_record, rng),
            Self::Metal(m) => m.scatter(r_in, hit_record, rng),
            Self::Dielectric(d) => d.scatter(r_in, hit_record, rng),
        }
    }
}

impl Default for MatKind {
    fn default() -> Self {
        MatKind::Lambertian(Lambertian::default())
    }
}
