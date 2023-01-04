#[path = "./vec3.rs"] mod vec3;
#[path = "./hit.rs"] mod hit;
#[path = "./ray.rs"] mod ray;

use crate::hit::{Hittable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool {
	let oc = r.orig - self.center;
	let a = r.dir.length_sqr();
	let halfb = oc.dot(&r.dir);
	let c = oc.length_sqr() - self.radius * self.radius;
	let discriminant = halfb*halfb - a*c;
	if discriminant < 0.0 { return false; } 

	let sqrtd = discriminant.sqrt();
	let mut root = (-halfb - sqrtd) / a;
	if root < tmin || tmax < root {
	    root = (-halfb + sqrtd) / a;
	    if root < tmin || tmax < root {
		return false;
	    }
	}

	rec.t = root;
	rec.point = r.at(rec.t);
	let outwnormal = (rec.point - self.center) / self.radius;
	rec.setfacenormal(r, &outwnormal);

	return true;
    }
}
