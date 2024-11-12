use core::f64;

use ray_tracing_in_one_weekend::{
    data3d::Color, data3d::Point3, data3d::Vec3, HitRecord, Hittable, HittableList, Ray, Sphere,
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);
    const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

    let lower_left_corner: Point3 =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray {
                origin: ORIGIN,
                direction: lower_left_corner - ORIGIN + u * HORIZONTAL + v * VERTICAL,
            };

            let pixel_color = ray_color(&ray, &world);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.")
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };
    if world.hit(ray, 0.0, f64::MAX, &mut rec) {
        return 0.5
            * Color::new(
                rec.normal.get_x() + 1.0,
                rec.normal.get_y() + 1.,
                rec.normal.get_z() + 1.,
            );
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
