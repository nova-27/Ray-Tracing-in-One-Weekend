use std::rc::Rc;

use crate::{
    data3d::{Point3, Vec3},
    material::Material,
    Ray,
};

pub mod sphere;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, material: Rc<dyn Material>) -> Self {
        Self {
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t,
            front_face: false,
            material,
        }
    }

    pub fn get_p(&self) -> Point3 {
        self.p
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }

    pub fn is_front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ret = None;
        let mut t_max = t_max;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, t_min, t_max) {
                t_max = record.t;
                ret = Some(record);
            }
        }

        ret
    }
}
