use crate::{
    definitions::{
        near_zero, random_double, random_in_unit_sphere, random_unit_vector, reflect, reflectance,
        refract,
    },
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

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, _scattered: &Ray) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0
            || reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0);

        let direction = if cannot_refract {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.point, direction)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Vec3;

    #[test]
    fn lambertian_test() {
        let l = Lambertian::new(Color::new(0.4, 0.5, 0.6));
        let on = Vec3::new(1.5, 0.1, 0.9);
        let r = Ray::new(on, on);
        let hr = HitRecord::default();
        let s = l.scatter(&r, &hr, &r);
        assert!(s.is_some());

        let (sc, _) = s.unwrap();
        assert_eq!(sc, Color::new(0.4, 0.5, 0.6));
    }

    #[test]
    fn metal_test() {
        let m = Metal::new(Color::new(0.4, 0.5, 0.6), 0.5);
        let on = Vec3::new(1.2, 1.0, 1.1);
        let r = Ray::new(on, on);
        let hr = HitRecord::default();
        let s = m.scatter(&r, &hr, &r);
        assert!(s.is_none());
    }

    #[test]
    fn dielectric_test() {
        let d = Dielectric::new(0.6);
        let on = Vec3::new(1.5, 0.1, 0.9);
        let r = Ray::new(on, on);
        let hr = HitRecord::default();
        let s = d.scatter(&r, &hr, &r);
        assert!(s.is_some());

        let (sc, _) = s.unwrap();
        assert_eq!(sc, Color::new(1.0, 1.0, 1.0));
    }
}
