use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use rand::Rng;

use camera::Camera;
use hit::hittable::Hittables;
use hit::sphere::Sphere;
use ray::point::Point3;
use ray::Ray;
use ray::vec3::Vec3;

use crate::ray::color::Color;
use crate::utils::rand;

mod camera;
mod ray;
mod hit;
mod utils;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 100.0;
    let max_depth = 50;

    //World
    let mut world = Hittables::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Image file
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut image_file = File::create("image.ppm").unwrap();
    image_file.write(header.as_bytes()).unwrap();

    for j in (0..(image_height as i32)).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..(image_width as i32) {
            let mut pixel_color = Color::default();

            for _ in 0..(samples_per_pixel as i32) {
                let u = (i as f64 + rand()) / (image_width - 1.0);
                let v = (j as f64 + rand()) / (image_height - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += r.color(&world, max_depth);
            }

            pixel_color.write_color(&mut image_file, samples_per_pixel);
        }
    }
    println!("DONE")
}
