use crate::{
    definitions::{near_zero, random_in_unit_sphere, random_unit_vector, reflect},
    Color, HitRecord, Ray,
};

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, _scattered: &Ray) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        Some((self.albedo.clone(), Ray::new(rec.point, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Option<(Color, Ray)> {
        let reflected = reflect(&r_in.direction().normalize(), &rec.normal);
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((
                self.albedo.clone(),
                Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere()),
            ))
        } else {
            None
        }
    }
}
