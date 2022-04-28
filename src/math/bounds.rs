#![allow(unused)]

use super::point::*;
use super::vec::*;
use crate::math;

#[derive(Clone, Copy, Debug)]
pub struct Bounds2 {
    p_min: Point2,
    p_max: Point2,
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds3 {
    p_min: Point3,
    p_max: Point3,
}

impl Bounds3 {
    #[inline(always)]
    pub fn new(p_min: Point3, p_max: Point3) -> Self {
        Self { p_min, p_max }
    }

    #[inline(always)]
    pub fn bound_single_point(p: Point3) -> Self {
        Bounds3::new(p, p)
    }

    #[inline(always)]
    pub fn from_points(p1: Point3, p2: Point3) -> Self {
        Bounds3::new(
            Point3::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z)),
            Point3::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z)),
        )
    }

    /// Yeah, I'm not really sure how to implement this since the
    /// C++ version relies on being able to index into a `Bounds3`,
    /// perform a bitwise & mask between the `corner` (`int` in the
    /// C++ code) and `1`, `2`, and `4` (for `x`, `y`, `z`,
    /// respectively). It uses this to do a ternary expression
    /// which I'm not sure what it is. So I just need to work this
    /// out on paper I think, and much like a lazy iterator, I don't
    /// plan to do so until I can see how this `corner` function gets
    /// used in practice.
    #[inline(always)]
    pub fn corner(&self, corner: usize) -> Point3 {
        unimplemented!()
    }

    #[inline(always)]
    pub fn union(self, p: Point3) -> Self {
        Self::new(
            Point3::new(
                self.p_min.x.min(p.x),
                self.p_min.y.min(p.y),
                self.p_min.z.min(p.z),
            ),
            Point3::new(
                self.p_max.x.max(p.x),
                self.p_max.y.max(p.y),
                self.p_max.z.max(p.z),
            ),
        )
    }

    ///     x--------------------O
    ///     |....................|
    ///     |..........x---------x------------------------O
    ///     |..........|+++++++++|........................|
    ///     O----------x---------x........................|
    ///                |..................................|
    ///                |..................................|
    ///                |..................................|
    ///                O----------------------------------x
    ///
    /// The shaded region represents the intersection of the two
    /// bounding box regions.
    #[inline(always)]
    pub fn intersect(self, b: Self) -> Self {
        Self::new(
            Point3::new(
                self.p_min.x.max(b.p_min.x),
                self.p_min.y.max(b.p_min.y),
                self.p_min.z.max(b.p_min.z),
            ),
            Point3::new(
                self.p_max.x.min(b.p_max.x),
                self.p_max.y.min(b.p_max.y),
                self.p_max.z.min(b.p_max.z),
            ),
        )
    }

    #[inline(always)]
    pub fn overlaps(self, b: Self) -> bool {
        let x = (self.p_max.x >= b.p_min.x) && (self.p_min.x <= b.p_max.x);
        let y = (self.p_max.y >= b.p_min.y) && (self.p_min.y <= b.p_max.y);
        let z = (self.p_max.z >= b.p_min.z) && (self.p_min.z <= b.p_max.z);

        x && y && z
    }

    /// Interestingly,they choose to include the boundary in their
    /// definition of the interior of a bounds, which to my mathematical
    /// eye isn't something that is quite proper, but I suppose it doesn't
    /// really matter here.
    #[inline(always)]
    pub fn inside(&self, p: &Point3) -> bool {
        p.x >= self.p_min.x
            && p.x <= self.p_max.x
            && p.y >= self.p_min.y
            && p.y <= self.p_max.y
            && p.z >= self.p_min.z
            && p.z <= self.p_max.z
    }

    #[inline(always)]
    pub fn expand(self, delta: f64) -> Self {
        Bounds3::new(
            self.p_min - Vec3::new(delta, delta, delta),
            self.p_max + Vec3::new(delta, delta, delta),
        )
    }

    #[inline(always)]
    pub fn diagonal(self) -> Vec3 {
        self.p_max - self.p_min
    }

    #[inline(always)]
    pub fn surface_area(self) -> f64 {
        let d = self.diagonal();

        2. * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    #[inline(always)]
    pub fn volume(self) -> f64 {
        let d = self.diagonal();

        d.x * d.y * d.z
    }

    /// Apparently this will be useful when I get around to writing
    /// an accelerator, but like all of this indexing into these structs
    /// nonsense, I will not implement it until I see how it's used.
    pub fn maximum_extent(self) -> usize {
        unimplemented!()
    }

    #[inline(always)]
    pub fn linterp(self, t: Point3) -> Point3 {
        Point3::new(
            math::linterp(t.x, self.p_min.x, self.p_max.x),
            math::linterp(t.y, self.p_min.y, self.p_max.y),
            math::linterp(t.z, self.p_min.z, self.p_max.z),
        )
    }

    #[inline(always)]
    pub fn offset(self, p: Point3) -> Vec3 {
        let mut o = p - self.p_min;
        let diff = self.p_max - self.p_min;

        if (diff.x > 0.) {
            o.x /= diff.x;
        }
        if (diff.y > 0.) {
            o.y /= diff.y;
        }
        if (diff.z > 0.) {
            o.z /= diff.z;
        }

        o
    }

    #[inline(always)]
    pub fn bounding_sphere(self) -> (Point3, f64) {
        let center = (self.p_min + self.p_max) / 2.;
        let radius = if self.inside(&center) {
            center.distance(self.p_max)
        } else {
            0.
        };

        (center, radius)
    }
}

/// We'll follow PBRT's lead here and make the default
/// something that isn't logically valid for a set of
/// bounding boxes. However, it seems worthwhile though
/// that this is revisited when I start using `Bounds3`
/// instances because initializing a mutable that is later
/// updated isn't a particularly idiomatic way to write
/// Rust code.
impl Default for Bounds3 {
    fn default() -> Self {
        let min = f64::MIN;
        let max = f64::MAX;

        let p_min = Point3::new(max, max, max);
        let p_max = Point3::new(min, min, min);

        Bounds3::new(p_min, p_max)
    }
}
