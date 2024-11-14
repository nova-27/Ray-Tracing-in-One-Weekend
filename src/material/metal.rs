use crate::{
    data3d::{Reflectance, Vec3},
    hittable::HitRecord,
    Ray,
};

use super::Material;

pub struct Metal {
    albedo: Reflectance,
}

impl Metal {
    pub fn new(albedo: Reflectance) -> Self {
        Self { albedo }
    }

    fn reflect(in_dir: Vec3, normal: Vec3) -> Vec3 {
        in_dir - 2.0 * in_dir.dot(&normal) * normal
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Reflectance)> {
        let reflected = Metal::reflect(ray_in.direction.unit_vector(), rec.get_normal());
        let scattered = Ray::new(rec.get_p(), reflected);
        if scattered.direction.dot(&rec.get_normal()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
