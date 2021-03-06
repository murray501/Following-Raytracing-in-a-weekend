use crate::myvec::Vec3;
use crate::ray::Ray;
use crate::hitable::{HitRecord, random_in_unit_sphere};
use rand::Rng;
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
} 


fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - n * (v.dot(n) * 2.0)
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        let refracted = (uv - n*dt)*ni_over_nt - n*discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, mut fuzz: f32) -> Self {
        if fuzz >= 1.0 {
            fuzz = 1.0;
        }
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }    
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) =
            if r_in.direction.dot(rec.normal) > 0.0 {
                let cosine = self.ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.length();
                (-rec.normal, self.ref_idx, cosine)
            } else {
                let cosine = -1.0 * r_in.direction.dot(rec.normal)/r_in.direction.length();
                (rec.normal, 1.0 / self.ref_idx, cosine)
            };
        
        match refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_idx);
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0.0..1.0) as f32;
                let scattered =
                    if random < reflect_prob {
                        Ray::new(rec.p, reflected)
                    } else {
                        Ray::new(rec.p, refracted)
                    };
                return Some((scattered, attenuation));
            }
            None => {
                    let scattered = Ray::new(rec.p, reflected);
                    return Some((scattered, attenuation));
                }
        }
    }    
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0)*(1.0 - cosine).powf(5.0);
}