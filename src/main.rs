use core::f64;
use std::rc::Rc;

use rand::Rng;
use ray_tracing_in_one_weekend::{
    camera::Camera,
    data3d::{Attenuation, Color, Point3},
    hittable::{sphere::Sphere, Hittable, HittableList},
    material::{lambertian::Lambertian, metal::Metal},
    Ray,
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: u32 = 50;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let camera = Camera::new(ASPECT_RATIO);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Attenuation::new(0.7, 0.3, 0.3))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Attenuation::new(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Attenuation::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Attenuation::new(0.8, 0.8, 0.8), 1.0)),
    )));

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            pixel_color.write_color(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done.")
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::MAX) {
        return match rec.get_material().scatter(ray, &rec) {
            Some((scattered, attenuation)) => attenuation * ray_color(&scattered, world, depth - 1),
            None => Color::new(0.0, 0.0, 0.0),
        };
    }

    let unit_direction = ray.get_direction().unit_vector();
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
