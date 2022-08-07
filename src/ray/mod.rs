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
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn color(&self, world: &Hittables, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(hit) = world.hit(self, 0.001, f64::INFINITY) {
            let target = hit.p + Vec3::random_in_hemisphere(&hit.normal);
            return 0.5 * Ray::new(hit.p, target - hit.p).color(world, depth - 1);
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
