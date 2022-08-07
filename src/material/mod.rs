use crate::{Color, Ray};
use crate::hit::hittable::HitRecord;

pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}