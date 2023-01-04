#[path = "./vec3.rs"] mod vec3;
use crate::vec3::Vec3;
#[path = "./ray.rs"] mod ray;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    lowerleft_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn init() -> Self {
	let aspect_ratio = 16.0 / 9.0;
	let vheight = 2.0;
	let wheight = aspect_ratio * vheight;
	let focallength = 1.0;

	let orig = Vec3::ZEROES;
	let h = Vec3::new(wheight, 0.0, 0.0);
	let v = Vec3::new(0.0, vheight, 0.0);
	let llcorner = orig - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, focallength);

	Camera {
	    origin: orig,
	    horizontal: h,
	    vertical: v,
	    lowerleft_corner: llcorner
	}
    }

    pub fn get_ray(self, u: f64, v: f64) -> Ray {
	Ray { orig: self.origin,
	      dir: { self.lowerleft_corner +
		     self.horizontal * u +
		     self.vertical * v -
		     self.origin
	      }
	}
    }
}
