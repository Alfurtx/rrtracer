use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};
use crate::vec3::Vec3;

use std::vec::Vec;
use std::boxed::Box;

pub struct HittableList<T: Hittable<T>> {
    pub objects: Vec<Box<T>>,
}

impl<T> Hittable<T> for HittableList<T> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord<T>) -> bool {
	// let mut temprec: HitRecord = HitRecord {point: rec.point, normal: rec.normal, t: rec.t, frontface: rec.frontface};
	let mut temprec: HitRecord = HitRecord {
	    point: Vec3::ZEROES,
	    normal: Vec3::ZEROES,
	    t: 0.0,
	    frontface: false,
	    mat: Option::None
	};

	let mut hitany = false;
	let mut closes_so_far = tmax;

	for obj in self.objects.iter() {
	    if obj.hit(r, tmin, closes_so_far, &mut temprec) {
		hitany = true;
		closes_so_far = temprec.t;
		rec.t = temprec.t;
		rec.point = temprec.point;
		rec.normal = temprec.normal;
		rec.frontface = temprec.frontface;
		rec.mat = temprec.mat;
	    }
	}

	return hitany;
    }
}

impl<T> HittableList<T> {
    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn add(&mut self, obj: Box<T>) {
	self.objects.push(obj);
    }
}
