use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use hit::hittable::Hittables;
use hit::sphere::Sphere;
use ray::point::Point3;
use ray::Ray;
use ray::vec3::Vec3;

mod ray;
mod hit;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;

    //World
    let mut world = Hittables::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -102.0, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut image_file = File::create("image.ppm").unwrap();
    image_file.write(header.as_bytes()).unwrap();

    for j in (0..(image_height as i32)).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..(image_width as i32) {
            let u = (i as f64) / (image_width - 1.0);
            let v = (j as f64) / (image_height - 1.0);
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color = r.color(&world);

            pixel_color.write_color(&mut image_file);
        }
    }
    println!("DONE")
}
