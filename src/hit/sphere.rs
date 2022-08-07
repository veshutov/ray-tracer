use crate::{Point3, Ray, Vec3};
use crate::hit::hittable::{HitRecord, Hittable};

#[derive(Default, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        return Self {
            center,
            radius,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // b*b*(t*t) + 2*b*(point-sphere_center)*(t) + (point-sphere_center)*(point-sphere_center) - sphere_radius * sphere_radius = 0
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;

        Some(disc)
            .filter(|d| *d > 0.0)
            .map(|d| (-half_b - d.sqrt()) / a)
            .filter(|t| *t > t_min && *t < t_max)
            .map(|t| {
                let outward_normal = (ray.at(t) - self.center) / self.radius;
                HitRecord::new(ray, &outward_normal, t)
            })
    }
}
