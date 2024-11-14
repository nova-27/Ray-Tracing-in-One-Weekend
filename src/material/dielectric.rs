use crate::{
    data3d::{Attenuation, Vec3},
    hittable::HitRecord,
    Ray,
};

use super::Material;

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }

    fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*uv).dot(normal);
        let r_out_parallel = etai_over_etat * (*uv + cos_theta * *normal);
        let r_out_perp = -((1.0 - r_out_parallel.length_squared()).sqrt()) * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Attenuation)> {
        let attenuation = Attenuation::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.is_front_face() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray_in.get_direction().unit_vector();
        let refracted = Self::refract(&unit_direction, &rec.get_normal(), etai_over_etat);
        let scattered = Ray::new(rec.get_p(), refracted);

        Some((scattered, attenuation))
    }
}
