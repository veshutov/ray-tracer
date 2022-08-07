use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use camera::Camera;
use hit::hittable::Hittables;
use hit::sphere::Sphere;
use material::lambertian::Lambertian;
use ray::color::Color;
use ray::point::Point3;
use ray::Ray;
use ray::vec3::Vec3;
use utils::rand;

use crate::material::metal::Metal;

mod camera;
mod ray;
mod hit;
mod utils;
mod material;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 100.0;
    let max_depth = 50;

    //World
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let mut world = Hittables::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

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
