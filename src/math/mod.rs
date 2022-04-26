pub mod normal;
pub mod point;
pub mod ray;
pub mod vec;

pub trait Dot<Rhs = Self> {
    fn dot(&self, rhs: &Rhs) -> f64;
}
