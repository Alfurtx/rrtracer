use std::ops::{Add, AddAssign, Neg, Sub, Mul, MulAssign, Div, DivAssign, Index};
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
	Self {x: a, y: b, z: c}
    }
    pub fn length_sqr(self) -> f64 {
	self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f64 {
	self.length_sqr().sqrt()
    }
    pub fn dot(self, other: &Vec3) -> f64 {
	self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(self, other: Vec3) -> Self {
	Self::new(self[1] * other[2] - self[2] * other[1],
	          self[2] * other[0] - self[0] * other[2],
	          self[0] * other[1] - self[1] * other[0])
    }
    pub fn unit_vector(self) -> Self {
	self / self.length()
    }

    pub fn newrandom() -> Self {
	Vec3::new(
	    rand::random::<f64>(),
	    rand::random::<f64>(),
	    rand::random::<f64>()
	)
    }
    pub fn newrandom_range(min: f64, max: f64) -> Self {
	let mut rng = rand::thread_rng();

	Vec3 {
	    x: rng.gen_range(min..max),
	    y: rng.gen_range(min..max),
	    z: rng.gen_range(min..max)
	}
    }

    pub fn random_in_unit_sphere() -> Self {
	loop {
	    let p = Vec3::newrandom_range(-1.0, 1.0);
	    if p.length_sqr() >= 1.0 { continue; }
	    return p;
	}
    }

    pub fn rand_unit_vec() -> Self {
	Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn rand_in_hemisphere(normal: &Vec3) -> Self {
	let inusphere = Vec3::random_in_unit_sphere();
	if inusphere.dot(normal) > 0.0 {
	    inusphere
	} else {
	    -inusphere
	}
    }

    pub fn near_zero(&self) -> bool {
	let s = 1e-8;
	self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: &Self, n: &Self) -> Self {
	*v - (*n * 2.0 * v.dot(n))
    }

    pub const ZEROES: Self = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONES: Self = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
}

pub fn zeros() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}
pub fn ones() -> Vec3 {
    Vec3::new(1.0, 1.0, 1.0)
}

impl Index<u32> for Vec3 {
    type Output = f64;
    fn index(&self, index: u32) -> &Self::Output {
	match index {
	    0 => &self.x,
	    1 => &self.y,
	    2 => &self.z,
	    _ => panic!("Out of bounds access to Vec3")
	}
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
	Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
	self.x += rhs.x;
	self.y += rhs.y;
	self.z += rhs.z;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
	self.x *= rhs;
	self.y *= rhs;
	self.z *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
	self.x /= rhs;
	self.y /= rhs;
	self.z /= rhs;
    }
}

impl Add for Vec3{
    type Output = Self;
    fn add(self, other: Self) -> Self {
	Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
	Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
	Self { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
	Self { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
	Self { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

