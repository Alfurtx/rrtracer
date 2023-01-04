#![allow(dead_code)]
#![allow(non_upper_case_globals)]

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

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
	return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
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
    r *= scale;
    g *= scale;
    b *= scale;

    // let i = (255.999 * r) as i32;
    // let j = (255.999 * g) as i32;
    // let k = (255.999 * b) as i32;

    println!("{} {} {}",
    	     (256.0 * clamp(r, 0.0, 0.999)) as i32,
    	     (256.0 * clamp(g, 0.0, 0.999)) as i32,
    	     (256.0 * clamp(b, 0.0, 0.999)) as i32
    );
    // println!("{} {} {}", i, j, k);
}

fn main() {
    // Image
    const aspect_ratio: f64 = 16.0 / 9.0;
    const image_width: u32 = 1000;
    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    const samples_per_pixel: u32 = 50;

    // World
    let mut world = HittableList { objects: Vec::new() };
    world.add(Box::new(Sphere {center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5}));
    world.add(Box::new(Sphere {center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}));

    // Camera
    let cam: Camera = Camera::init();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
	eprint!("{esc}c", esc = 27 as char); // clear the screen
	eprintln!("Scanlines remaining: {}", j);
	for i in 0..image_width {
	    let mut color_pixel = Vec3::ZEROES;
	    for _s in 0..samples_per_pixel {
	    	let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
	    	let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
	    	let r = cam.get_ray(u, v);
	    	color_pixel += ray_color(&r, &world);
	    }
	    write_color(color_pixel, samples_per_pixel);
	}
    }
    eprintln!("DONE");
}
