use crate::ray::Ray;
use crate::render::Point;
use crate::sphere::Sphere;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(p: Point, normal: Vec3, t: f64) -> Self {
        HitRecord { p, normal, t }
    }

    pub const fn empty() -> Self {
        Self { p: Point::ORIGIN, normal: Vec3::X_HAT, t: 0.0 }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum Hittable {
    Sphere(Sphere),
}

impl Hit for Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self {
            Self::Sphere(s) => return s.hit(ray, t_min, t_max, rec)
        }
    }
}

pub type HitList = Vec<Hittable>;

impl Hit for HitList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hit in self.iter() {
            if hit.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
            }
        }

        hit_anything
    }
}