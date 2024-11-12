use crate::{
    data3d::{Point3, Vec3},
    Ray,
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const FOCAL_LENGTH: f64 = 1.0;
        let viewport_width: f64 = aspect_ratio * VIEWPORT_HEIGHT;

        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner: Point3 =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
