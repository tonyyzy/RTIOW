use std::rc::Rc;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.normal = if r.direction().dot(outward_normal) < 0.0 {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord)
        -> bool;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closet_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closet_so_far, &mut temp_rec) {
                hit_anything = true;
                closet_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
