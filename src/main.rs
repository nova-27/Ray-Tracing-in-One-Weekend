use core::f64;

use ray_tracing_in_one_weekend::{
    camera::Camera,
    data3d::{Color, Point3},
    Hittable, HittableList, Ray, Sphere,
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let camera = Camera::new(ASPECT_RATIO);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = camera.get_ray(u, v);

            let pixel_color = ray_color(&ray, &world);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.")
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::MAX) {
        return 0.5
            * Color::new(
                rec.get_normal().get_x() + 1.0,
                rec.get_normal().get_y() + 1.,
                rec.get_normal().get_z() + 1.,
            );
    }

    let unit_direction = ray.get_direction().unit_vector();
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
