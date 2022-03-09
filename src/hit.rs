use crate::ray::Ray;
use crate::render::Point;
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
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
