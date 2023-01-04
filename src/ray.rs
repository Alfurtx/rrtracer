#[path = "./vec3.rs"] mod vec3;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
	Ray { orig: origin, dir: direction}
    }
    pub fn at(self, t: f64) -> Vec3 {
	self.orig + self.dir * t
    }
}
