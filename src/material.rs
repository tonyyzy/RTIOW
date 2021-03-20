use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_in_unit_sphere, random_unit_vector, Color},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, fuzz: f64) -> Self {
        Self {
            albedo: a,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        *scattered =
            Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo.clone();
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
