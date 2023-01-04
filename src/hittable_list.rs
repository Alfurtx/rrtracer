#[path = "./ray.rs"] mod ray;
use crate::ray::Ray;
#[path = "./hit.rs"] mod hit;
use crate::hit::{HitRecord, Hittable};
#[path = "./vec3.rs"] mod vec3;
use crate::vec3::Vec3;

use std::vec::Vec;
use std::boxed::Box;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool {
	// let mut temprec: HitRecord = HitRecord {point: rec.point, normal: rec.normal, t: rec.t, frontface: rec.frontface};
	let mut temprec: HitRecord = HitRecord {
	    point: Vec3::ZEROES,
	    normal: Vec3::ZEROES,
	    t: 0.0,
	    frontface: false
	};

	let mut hitany = false;
	let mut closes_so_far = tmax;

	for obj in self.objects.iter() {
	    if obj.hit(r, tmin, closes_so_far, &mut temprec) {
		hitany = true;
		closes_so_far = temprec.t;
		*rec = temprec;
	    }
	}

	hitany
    }
}

impl HittableList {
    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
	self.objects.push(obj);
    }
}
