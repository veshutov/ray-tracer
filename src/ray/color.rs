use std::fs::File;
use std::io::Write;

use crate::ray::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, image_file: &mut File) {
        let line = format!(
            "{} {} {}\n",
            (255.999 * self.x) as i32,
            (255.999 * self.y) as i32,
            (255.999 * self.z) as i32,
        );
        image_file.write(line.as_bytes()).unwrap();
    }
}