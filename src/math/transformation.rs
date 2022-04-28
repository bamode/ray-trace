#![allow(unused)]

use super::vec::Vec3;

trait Transpose {
    fn transpose(self) -> Self;
}

pub struct Transformation {
    matrix: [[f64; 4]; 4],
}

impl Transformation {
    #[inline(always)]
    pub fn new(matrix: [[f64; 4]; 4]) -> Self {
        Self { matrix }
    }
}

impl Transpose for Transformation {
    #[inline(always)]
    fn transpose(self) -> Self {
        let m = self.matrix;
        Self::new([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]]
        ])
    }
}

/// Being able to initialize a mutable 0 matrix is often
/// pretty useful, at least in terms of initially
/// implementing algorithms in the style that they're
/// normally described in pseudocode, which usually mirrors
/// something (dreadful) like C++ or Java.
impl Default for Transformation {
    fn default() -> Self {
        Transformation::new([
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
        ])
    }
}

pub struct Transform {
    transform: Transformation,
    inverse: Transformation,
}

impl Transform {
    /// This method does not guarantee that the inverse is
    /// correctly constructed such that T * T ^ (-1) = I.
    /// The motivation behind this design decision is that it will
    /// often be unnecessary to calculate the inverse at runtime, so
    /// it would be relatively costly to automatically compute the
    /// transform's inverse. Instead a method `Self::check_inverse`
    /// will be available to check if the transform's inverse is
    /// correct.
    #[inline(always)]
    pub fn new(t: Transformation, t_inv: Transformation) -> Self {
        Self {
            transform: t,
            inverse: t_inv,
        }
    }

    /// Again, we assume that the library end user has ensured that the
    /// inverse is actually the inverse (within some necessary precision)
    #[inline(always)]
    pub fn inverse(self) -> Self {
        Self::new(self.inverse, self.transform)
    }

    #[inline(always)]
    pub fn translate(delta: Vec3) -> Self {
        Self::new(
            Transformation::new([
                [1., 0., 0., delta.x],
                [0., 1., 0., delta.y],
                [0., 0., 1., delta.z],
                [0., 0., 0.,      1.]
            ]),
            Transformation::new([
                [1., 0., 0., -delta.x],
                [0., 1., 0., -delta.y],
                [0., 0., 1., -delta.z],
                [0., 0., 0.,       1.]
            ])
        )
    }

    #[inline(always)]
    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Self::new(
            Transformation::new([
                [x , 0., 0., 0.],
                [0., y , 0., 0.],
                [0., 0., z , 0.],
                [0., 0., 0., 1.]
            ]),
            Transformation::new([
                [1./x ,   0. ,   0. ,  0.  ],
                [  0. , 1./y ,   0. ,  0.  ],
                [  0. ,   0. , 1./z ,  0.  ],
                [  0. ,   0. ,   0. ,  1.  ]
            ])
        )
    }

    /// This method should be called on any `Transform` that is constructed
    /// from user input or has not been mathematically verified prior to
    /// compile time. Ideally, this should probably return a 
    /// `Result<(), InvalidInverse>` where `InvalidInverse` implements
    /// `std::error::Error`, but for now, it's probably easier to deal with
    /// a bool that returns `true` for a valid inverse and `false` for an invalid
    /// one. That does sort of raise a question though of perhaps a more clever
    /// way to handle an invalid inverse: 
    /// 
    /// - Suppose that the transformation is correct and the transformation
    ///   inverse exists. 
    /// - Return a `Result<(), Transformation>` or something like that where
    ///   the error case is handled by pattern matching and the function
    ///   simply calculates the inverse for the user. 
    #[inline(always)]
    pub fn check_inverse_valid(&self) -> bool {
        todo!()
    }
}

impl Transpose for Transform {
    #[inline(always)]
    fn transpose(self) -> Self {
        Self::new(
            Transformation::new(self.transform.transpose().matrix),
            Transformation::new(self.inverse.transpose().matrix)
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transformation_transpose() {
        let a = [[0., 1., 0., 0.],
                 [0., 0., 0., 0.],
                 [0., 0., 0., 0.],
                 [0., 0., 0., 0.]];
        
        let a_inv = [[0., 0., 0., 0.],
                     [1., 0., 0., 0.],
                     [0., 0., 0., 0.],
                     [0., 0., 0., 0.]];

        assert_eq!(
            a_inv,
            Transformation::new(a).transpose().matrix
        );
    }
}