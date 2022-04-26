use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::math::ray::Ray;
use crate::render::Point;
use crate::math::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Sphere<Mat>
where
    Mat: Material + Copy + Default,
{
    pub center: Point,
    pub r: f64,
    pub material: Mat,
}

impl<Mat> Sphere<Mat>
where
    Mat: Material + Copy + Default,
{
    pub fn new(center: Point, r: f64, material: Mat) -> Self {
        Sphere {
            center,
            r,
            material,
        }
    }
}

impl<Mat> Hit<Mat> for Sphere<Mat>
where
    Mat: Material + Copy + Default,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord<Mat>) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.length_squared() - self.r * self.r;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a; // this is the quadratic formula
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = ((p - self.center) / self.r).unit_vector();

        hit_record.p = p;
        hit_record.normal = normal;
        hit_record.t = t;

        let outward_normal: Vec3 = (hit_record.p - self.center) / self.r;
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = self.material;
        true
    }
}
