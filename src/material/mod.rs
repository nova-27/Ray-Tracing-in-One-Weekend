use crate::{data3d::Attenuation, hittable::HitRecord, Ray};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Attenuation)>;
}
