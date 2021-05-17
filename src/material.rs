use crate::{definitions::{near_zero, random_unit_vector, reflect}, Color, HitRecord, Ray};

pub trait Material: Sync + Send {
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

impl Lambertian{
    pub fn new() ->Self {
        Self {
            albedo:Color::new(0.0,0.0,0.0),
        }
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
        let scatter_direction = rec.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        scattered = &mut Ray::new(rec.point, random_unit_vector());
        attenuation = &mut self.albedo.clone();
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction().normalize(),&rec.normal);
        scattered = &mut Ray::new(rec.point, random_unit_vector());
        attenuation = &mut self.albedo.clone();
        scattered.direction().dot(&rec.normal) > 0.0
    }
}