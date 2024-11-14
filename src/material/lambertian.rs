use crate::{
    data3d::{Reflectance, Vec3},
    hittable::HitRecord,
    Ray,
};

use super::Material;

pub struct Lambertian {
    albedo: Reflectance,
}

impl Lambertian {
    pub fn new(albedo: Reflectance) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Reflectance)> {
        let scatter_direction = rec.get_normal() + Vec3::random_unit_sphere();
        let scattered = Ray::new(rec.get_p(), scatter_direction);
        Some((scattered, self.albedo))
    }
}
