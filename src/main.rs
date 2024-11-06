use ray_tracing_in_one_weekend::{Color, Point3, Ray, Vec3};

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

    const ORIGIN: Point3 = Point3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    const HORIZONTAL: Vec3 = Vec3 {
        x: VIEWPORT_WIDTH,
        y: 0.,
        z: 0.,
    };
    const VERTICAL: Vec3 = Vec3 {
        x: 0.,
        y: VIEWPORT_HEIGHT,
        z: 0.,
    };

    let lower_left_corner: Point3 = ORIGIN
        - HORIZONTAL / 2.
        - VERTICAL / 2.
        - Vec3 {
            x: 0.,
            y: 0.,
            z: FOCAL_LENGTH,
        };

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray {
                origin: ORIGIN,
                direction: lower_left_corner - ORIGIN + u * HORIZONTAL + v * VERTICAL,
            };

            let pixel_color = ray_color(&ray);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.")
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(
        &Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        0.5,
        ray,
    ) {
        return Color {
            r: 1.,
            g: 0.,
            b: 0.,
        };
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t)
        * Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
        + t * Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let half_b = Vec3::dot(&oc, &ray.direction);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    half_b * half_b - a * c > 0.
}
