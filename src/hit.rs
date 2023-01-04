#[path = "./vec3.rs"] mod vec3;
use crate::vec3::Vec3;

#[path = "./ray.rs"] mod ray;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub frontface: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn setfacenormal(&mut self, r: &Ray, outward_normal: &Vec3) {
	self.frontface = r.dir.dot(outward_normal) < 0.0;
	self.normal = {
	    if self.frontface {
		*outward_normal
	    } else {
		-*outward_normal
	    }
	}
    }

    pub fn default() -> Self {
	HitRecord { point: Vec3::ZEROES, normal: Vec3::ZEROES, t: 0.0, frontface: false }
    }
}
