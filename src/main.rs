#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use rand::prelude::*;

mod lambertian;
use crate::lambertian::Lambertian;

mod material;
use crate::material::*;

mod vec3;
use crate::vec3::Vec3;

mod ray;
use crate::ray::Ray;

mod hit;
use crate::hit::{HitRecord, Hittable};

mod sphere;
use crate::sphere::Sphere;

mod hittable_list;
use crate::hittable_list::HittableList;

mod camera;
use crate::camera::Camera;

// const IMAGE_WIDTH: u32 = 256;
// const IMAGE_HEIGHT: u32 = 256;

fn degrees_to_rads(deg: f64) -> f64 { deg * std::f64::consts::PI / 180.0 }

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    else if x > max { return max; }
    else { return x; }
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.length_sqr();
    let halfb = oc.dot(&r.dir);
    let c = oc.length_sqr() - radius * radius;
    let discriminant = halfb*halfb - a*c;
    if discriminant < 0.0 {
	-1.0
    } else {
	(-halfb - discriminant.sqrt()) / a
    }
}

fn ray_color<T>(r: &Ray, world: &dyn Hittable<T>, depth: i32) -> Vec3 {
    let mut rec = HitRecord {
	point: Vec3::ZEROES,
	normal: Vec3::ZEROES,
	t: 0.0,
	frontface: false,
	mat: Option::<Box<dyn Material>>::None
    };

    if depth <= 0 {
	return Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
	let mut scattered = Ray::new(Vec3::ZEROES, Vec3::ZEROES);
	let mut attenuation = Vec3::ZEROES;
	// if rec.mat.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
	//     return attenuation * ray_color(&scattered, world, depth - 1);
	// }
	return Vec3::ZEROES;
    }

    let ud = r.dir.unit_vector();
    let t = 0.5 * (ud.y + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn write_color(vec: Vec3, samplesperpixel: u32) {
    let mut r = vec.x;
    let mut g = vec.y;
    let mut b = vec.z;

    let scale = 1.0 / samplesperpixel as f64;
    r = (r *scale).sqrt();
    g = (g *scale).sqrt();
    b = (b *scale).sqrt();

    println!("{} {} {}",
    	     (256.0 * clamp(r, 0.0, 0.999)) as i32,
    	     (256.0 * clamp(g, 0.0, 0.999)) as i32,
    	     (256.0 * clamp(b, 0.0, 0.999)) as i32
    );
}

fn main() {

    // Image
    const aspect_ratio: f64 = 16.0 / 9.0;
    const image_width: u32 = 1000;
    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    const samples_per_pixel: u32 = 4;
    const max_depth: i32 = 50;

    // World
    let mut world = HittableList { objects: Vec::new() };
    let matground = Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) };
    let matcenter = Lambertian { albedo: Vec3::new(0.7, 0.3, 0.3) };
    let matleft = Metal { albedo: Vec3::new(0.8, 0.8, 0.8) };
    let matright = Metal { albedo: Vec3::new(0.8, 0.6, 0.2) };

    world.add(Box::new(Sphere {
	center: Vec3::new(0.0, 0.0, -1.0),
	radius: 0.5,
	mat: Option::Some(Box::new(matcenter))
    }));
    world.add(Box::new(Sphere {
	center: Vec3::new(0.0, -100.5, -1.0),
	radius: 100.0,
	mat: Option::Some(Box::new(matground))
    }));
    world.add(Box::new(Sphere {
	center: Vec3::new(-1.0, 0.0, -1.0),
	radius: 0.5,
	mat: Option::Some(Box::new(matleft))
    }));
    world.add(Box::new(Sphere {
	center: Vec3::new(1.0, 0.0, -1.0),
	radius: 0.5,
	mat: Option::Some(Box::new(matright))
    }));

    // Camera
    let cam: Camera = Camera::init();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
	eprint!("{esc}c", esc = 27 as char); // clear the screen
	eprintln!("Scanlines remaining: {}", j);
	for i in 0..image_width {
	    let mut color_pixel = Vec3::ZEROES;
	    for _s in 0..samples_per_pixel {
	    	let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
	    	let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
	    	let r = cam.get_ray(u, v);
	    	color_pixel += ray_color(&r, &world, max_depth);
	    }
	    write_color(color_pixel, samples_per_pixel);
	}
    }
    eprintln!("DONE");
}
