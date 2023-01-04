#[path = "./vec3.rs"] mod vec3;
use crate::vec3::Vec3;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f64,
}
