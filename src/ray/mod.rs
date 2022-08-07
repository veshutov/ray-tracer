use crate::hit::hittable::Hittable;
use crate::hit::hittable::Hittables;
use crate::ray::color::Color;
use crate::ray::point::Point3;
use crate::ray::vec3::Vec3;

pub mod vec3;
pub mod point;
pub mod color;

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn color(&self, world: &Hittables) -> Color {
        if let Some(hit) = world.hit(self, 0.0, f64::INFINITY) {
            return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
