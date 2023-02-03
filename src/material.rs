use crate::vec3::Vec3;
use crate::hit::HitRecord;
use crate::ray::Ray;

pub trait Material: Sized {
    // type Object;
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<Self>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vec3
}

impl Material for Metal {
    // type Object = Metal;
    fn scatter(&self,
	       r_in: &Ray,
	       rec: &HitRecord<Self>,
	       attenuation: &mut Vec3,
	       scattered: &mut Ray)
	       -> bool
    {
	let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
	scattered.orig = rec.point;
	scattered.dir = reflected;
	attenuation.x = self.albedo.x;
	attenuation.y = self.albedo.y;
	attenuation.z = self.albedo.z;

	return scattered.dir.dot(&rec.normal) > 0.0
    }
}
