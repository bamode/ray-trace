use crate::render::Point;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Point,
}

impl Ray {
    pub fn new(origin: Point, dir: Point) -> Self {
        Self { origin, dir }
    }
    
    #[inline]
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
}
