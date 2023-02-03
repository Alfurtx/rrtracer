use crate::hit::{Hittable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Option<Box<T>>
}

impl<T: Material> Hittable<T> for Sphere<T> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord<T>) -> bool {
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

	let res = self.mat.unwrap();

	rec.t = root;
	rec.point = r.at(rec.t);
	let outwnormal = (rec.point - self.center) / self.radius;
	rec.setfacenormal(r, &outwnormal);
	rec.mat = self.mat;

	return true;
    }
}
