use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3
}

impl Material for Lambertian {
    // type Object = Lambertian;
    fn scatter(&self,
	       r_in: &Ray,
	       rec: &HitRecord<Self>,
	       attenuation: &mut Vec3,
	       scattered: &mut Ray)
	       -> bool
    {
	let mut scatter_dir = rec.normal + Vec3::rand_unit_vec();

	if scatter_dir.near_zero() {
	    scatter_dir = rec.normal;
	}

	scattered.orig = rec.point;
	scattered.dir = scatter_dir;
	attenuation.x = self.albedo.x;
	attenuation.y = self.albedo.y;
	attenuation.z = self.albedo.z;

	return true;
    }
}
