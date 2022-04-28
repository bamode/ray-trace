pub mod bounds;
pub mod normal;
pub mod point;
pub mod ray;
pub mod transformation;
pub mod vec;

pub trait Dot<Rhs = Self> {
    fn dot(&self, rhs: &Rhs) -> f64;
}

#[inline(always)]
fn linterp(t: f64, v1: f64, v2: f64) -> f64 {
    (1. - t) * v1 + t * v2
}
