use std::fs::File;
use std::io::Write;

use crate::ray::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, image_file: &mut File, samples_per_pixel: f64) {
        let scale = 1.0 / samples_per_pixel;
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        let line = format!(
            "{} {} {}\n",
            (255.999 * clamp(r, 0.0, 0.999)) as i32,
            (255.999 * clamp(g, 0.0, 0.999)) as i32,
            (255.999 * clamp(b, 0.0, 0.999)) as i32,
        );
        image_file.write(line.as_bytes()).unwrap();
    }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}