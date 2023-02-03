use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

use std::boxed::Box;

/*
 * HIT
 */

// #[derive(Debug, Clone, Copy)]
pub struct HitRecord<T: Material> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub frontface: bool,
    pub mat: Option<Box<T>>
}

pub trait Hittable<T: Material> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord<T>) -> bool;
}

impl<T: Material> HitRecord<T> {
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
}
