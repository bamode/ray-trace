use crate::material::{Material, Lambertian};
use crate::ray::Ray;
use crate::render::Point;
use crate::sphere::Sphere;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord<Mat> 
where
    Mat: Material + Copy + Default,
{
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub material: Mat,
    pub front_face: Option<bool>,
}

impl<Mat> HitRecord<Mat>
where
    Mat: Material + Copy + Default,
{
    pub fn new(p: Point, normal: Vec3, t: f64, material: Mat, front_face: Option<bool>) -> Self {
        HitRecord { p, normal, t, material, front_face }
    }

    pub fn empty() -> Self {
        Self::new(Point::ORIGIN, Vec3::X_HAT, 0.0, Mat::default(), None)
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Some(ray.dir.dot(outward_normal) < 0.0);
        match self.front_face { 
            Some(true) => { self.normal = *outward_normal; } 
            Some(false) => { self.normal = -(*outward_normal); }
            None => panic!("Impossible to reach error")
        }
    }
}

pub trait Hit<Mat>
where
    Mat: Material + Copy + Default
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<Mat>) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum Hittable<Mat> 
where
    Mat: Material + Copy + Default
{
    Sphere(Sphere<Mat>),
}

impl<Mat> Hit<Mat> for Hittable<Mat> 
where
    Mat: Material + Copy + Default
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<Mat>) -> bool {
        match self {
            Self::Sphere(s) => return s.hit(ray, t_min, t_max, rec)
        }
    }
}

pub struct HitList<Mat>
where
    Mat: Material + Copy + Default
{
    inner: Vec<Hittable<Mat>>,
}

impl<Mat> HitList<Mat>
where
    Mat: Material + Copy + Default
{
    pub fn new() -> Self {
        HitList { inner: Vec::new() }
    }

    pub fn push(&mut self, hittable: Hittable<Mat>) {
        self.inner.push(hittable)
    }
}

impl<Mat> Hit<Mat> for HitList<Mat> 
where
    Mat: Material + Copy + Default
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<Mat>) -> bool {
        let mut temp_rec: HitRecord<Mat> = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hit in self.inner.iter() {
            if hit.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                //rec.p = temp_rec.p;
                //rec.normal = temp_rec.normal;
                //rec.t = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
