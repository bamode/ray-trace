use crate::hit::{HitRecord, Hit};
use crate::ray::Ray;
use crate::render::Point;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Point,
    pub r: f64,
}

impl Sphere {
    pub fn new(center: Point, r: f64) -> Self {
        Sphere { center, r }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.length_squared() - self.r * self.r;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return false }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let root = (-half_b - sqrtd) / a; // this is the quadratic formula
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        let t = root;
        let p = ray.at(root);
        let normal = ((p - self.center) / self.r).unit_vector(); 
        hit_record.p = p;
        hit_record.normal = normal;
        hit_record.t = t;

        true
    }
}
