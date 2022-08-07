use crate::{Color, Ray, Vec3};
use crate::hit::hittable::HitRecord;
use crate::material::Material;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal {
            albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.direction.unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        let attenuation = self.albedo;
        return if Vec3::dot(&scattered.direction, &hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        };
    }
}