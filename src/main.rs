#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod vec3;
use crate::vec3::Vec3;

mod ray;
use crate::ray::Ray;

// const IMAGE_WIDTH: u32 = 256;
// const IMAGE_HEIGHT: u32 = 256;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.dot(r.dir);
    let b = oc.dot(r.dir) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
	-1.0
    } else {
	(-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Vec3 {
    let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
	let v = r.at(t) - Vec3::new(0.0, 0.0, -1.0);
	let n = v.unit_vector();
	return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
    }

    let ud = r.dir.unit_vector();
    t = 0.5 * (ud.y + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn write_color(vec: Vec3) {
    let i = (255.999 * vec.x) as i32;
    let j = (255.999 * vec.y) as i32;
    let k = (255.999 * vec.z) as i32;

    println!("{} {} {}", i, j, k);
}

fn main() {
    // Image
    const image_width: u32 = 1980;
    const image_height: u32 = 1080;
    const aspect_ratio: f64 = 16.0 / 9.0;

    // Camera
    let viewporth = 2.0;
    let viewportw = aspect_ratio * viewporth;
    let focal_len = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewportw, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewporth, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_len);

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
	eprint!("{esc}c", esc = 27 as char); // clear the screen
	eprintln!("Scanlines remaining: {}", j);
	for i in 0..image_width {
	    let u = (i as f64) / (image_width - 1) as f64;
	    let v = (j as f64) / (image_height - 1) as f64;
	    let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
	    let pixel_color = ray_color(&r);
	    write_color(pixel_color);
	}
    }
    eprintln!("DONE");
}
