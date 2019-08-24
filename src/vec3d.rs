#![allow(dead_code)]
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use std::{cmp, fmt};

pub fn lerp(a: Vec3d, b: Vec3d, d: f64) -> Vec3d {
    a + (b - a) * d.max(0.).min(1.)
}

#[derive(Copy, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn x_comp(self) -> Vec3d {
        Vec3d::new(self.x, 0., 0.)
    }

    pub fn y_comp(self) -> Vec3d {
        Vec3d::new(0., self.y, 0.)
    }

    pub fn z_comp(self) -> Vec3d {
        Vec3d::new(0., 0., self.z)
    }

    pub fn new(x: impl Scalar, y: impl Scalar, z: impl Scalar) -> Vec3d {
        Vec3d {
            x: x.float(),
            y: y.float(),
            z: z.float(),
        }
    }

    pub fn length(&self) -> f64 {
        eucl(self.x, self.y, self.z)
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn zero() -> Vec3d {
        Vec3d::new(0., 0., 0.)
    }

    pub fn one() -> Vec3d {
        Vec3d::new(1., 1., 1.)
    }

    pub fn scale(&self, scalar: f64) -> Vec3d {
        Vec3d::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn lerp(v1: Vec3d, v2: Vec3d, alpha: f64) -> Vec3d {
        v1 + (v2 - v1) * alpha.max(0.).min(1.)
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec3d {
        Vec3d::new(
            self.x.max(min).min(max),
            self.y.max(min).min(max),
            self.z.max(min).min(max),
        )
    }

    pub fn distance(&self, v: Vec3d) -> f64 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
        eucl(dx, dy, dz)
    }

    pub fn normalize(&mut self) {
        let norm = eucl(self.x, self.y, self.z);
        self.x /= norm;
        self.y /= norm;
        self.z /= norm;
    }

    pub fn normalized(&self) -> Vec3d {
        let scale = 1. / self.length();
        Vec3d::new(self.x * scale, self.y * scale, self.z * scale)
    }

    pub fn normalized_by(&self, frac: f64) -> Vec3d {
        let scale = frac / self.length();
        Vec3d::new(self.x * scale, self.y * scale, self.z * scale)
    }

    pub fn dot(&self, vec: Vec3d) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(self, vec: Vec3d) -> Vec3d {
        Vec3d::new(
            self.y * vec.z - self.z * vec.y,
            self.z * vec.x - self.x * vec.z,
            self.x * vec.y - self.y * vec.x,
        )
    }

    fn vec_from_angle(theta: f64, phi: f64) -> Vec3d {
        Vec3d::new(
            theta.sin() * phi.cos(),
            theta.sin() * phi.sin(),
            theta.cos(),
        )
    }

    pub fn angle2d(self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn angle(self) -> (f64, f64) {
        (self.y.atan2(self.x), self.z.cos())
    }

    pub fn recip(self) -> Vec3d {
        Vec3d::new(self.x.recip(), self.y.recip(), self.z.recip())
    }

    pub fn min(self, other: impl Into<Vec3d>) -> Vec3d {
        let other = other.into();
        Vec3d::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn max(self, other: impl Into<Vec3d>) -> Vec3d {
        let other = other.into();
        Vec3d::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    pub fn as_slice(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

pub fn eucl(x: impl Scalar, y: impl Scalar, z: impl Scalar) -> f64 {
    let x = x.float();
    let y = y.float();
    let z = z.float();
    (x * x + y * y + z * z).sqrt()
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, other: Vec3d) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<f64> for Vec3d {
    type Output = Vec3d;

    fn add(self, other: f64) -> Vec3d {
        Vec3d::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<f64> for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: f64) -> Vec3d {
        Vec3d::new(self.x - other, self.y - other, self.z - other)
    }
}

impl Mul for Vec3d {
    type Output = f64;

    fn mul(self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: f64) -> Vec3d {
        Vec3d::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Div for Vec3d {
    type Output = Vec3d;

    fn div(self, other: Vec3d) -> Vec3d {
        Vec3d::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, other: f64) -> Vec3d {
        Vec3d::new(self.x / other, self.y / other, self.z / other)
    }
}

impl Neg for Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Vec3d {
        Vec3d::new(-self.x, -self.y, -self.z)
    }
}

impl cmp::PartialEq for Vec3d {
    fn eq(&self, other: &Vec3d) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Debug for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Scalar, U: Scalar> From<(T, U)> for Vec3d {
    fn from(other: (T, U)) -> Vec3d {
        Vec3d::new(other.0, other.1, 0.)
    }
}

impl<T: Scalar> From<[T; 2]> for Vec3d {
    fn from(other: [T; 2]) -> Vec3d {
        Vec3d::new(other[0], other[1], 0.)
    }
}

impl<T: Scalar, U: Scalar, Y: Scalar> From<(T, U, Y)> for Vec3d {
    fn from(other: (T, U, Y)) -> Vec3d {
        Vec3d::new(other.0, other.1, other.2)
    }
}

impl<T: Scalar> From<[T; 3]> for Vec3d {
    fn from(other: [T; 3]) -> Vec3d {
        Vec3d::new(other[0], other[1], other[2])
    }
}

impl std::ops::Index<usize> for Vec3d {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bounds in Vec3d"),
        }
    }
}

// impl<T: Scalar> From<cgmath::Point2<T>> for Vec3d {
//     fn from(other: cgmath::Point2<T>) -> Vec3d {
//         Vec3d::new2d(other[0], other[1])
//     }
// }

// impl<T: Scalar> From<cgmath::Point3<T>> for Vec3d {
//     fn from(other: cgmath::Point3<T>) -> Vec3d {
//         Vec3d::new(other[0], other[1], other[2])
//     }
// }

// impl<T: Scalar+std::fmt::Debug+std::cmp::PartialEq> From<nalgebra::geometry::Point2<T>> for Vec3d {
//     fn from(other: nalgebra::geometry::Point3<T>) -> Vec3d {
//         Vec3d::new2d(other[0], other[1])
//     }
// }

// impl<T: Scalar> From<nalgebra::geometry::Point3<T>> for Vec3d {
//     fn from(other: nalgebra::geometry::Point3<T>) -> Vec3d {
//         Vec3d::new(other[0], other[1], other[2])
//     }
// }

pub trait Scalar: Copy {
    fn float(self) -> f64;
}

impl Scalar for u8 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for u16 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for u32 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for i8 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for i16 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for i32 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}

impl Scalar for f32 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}

impl Scalar for f64 {
    fn float(self) -> f64 {
        self
    }
}
