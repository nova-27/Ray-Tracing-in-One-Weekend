use crate::{data3d::Reflectance, hittable::HitRecord, Ray};

pub mod lambertian;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Reflectance)>;
}
