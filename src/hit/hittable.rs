use std::rc::Rc;

use crate::{Point3, Ray, Vec3};
use crate::material::Material;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(ray: &Ray, outward_normal: &Vec3, t: f64, material: Rc<dyn Material>) -> HitRecord {
        let front_face = Vec3::dot(&ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        };
        HitRecord {
            t,
            p: ray.at(t),
            normal,
            front_face,
            material,
        }
    }
}

pub struct Hittables {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new() -> Hittables {
        Hittables {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable)
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result_hit = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result_hit = Some(hit);
            }
        }
        return result_hit;
    }
}