use crate::myvec::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use std::rc::Rc;
use rand::Rng;
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_paramter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {t, p, normal, material: Rc::clone(&self.material)});
            }
            let temp = (-b + (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_paramter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{t, p, normal, material: Rc::clone(&self.material)});
            }
        }
        return None;
    }
}

#[derive(Default)]
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest: Option<HitRecord> = None;
        for hitable in self.list.iter() {
            if let Some(rec) =  hitable.hit(r,t_min, closest_so_far) {
                closest_so_far = rec.t;
                closest = Some(rec);
            }
        }
        return closest;
    }    
}
    
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(-1.0..1.0) as f32;
        let y = rng.gen_range(-1.0..1.0) as f32;
        let z = rng.gen_range(-1.0..1.0) as f32;
        let p = Vec3::new(x,y,z);
        if p.length() < 1.0 {
            return p;
        }
    }
}

pub fn color(r: &Ray, world: &HitableList, depth: usize) -> Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            if depth < 50 {
                if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
                    return attenuation * color(&scattered, world, depth + 1);
                }
            }
            return Vec3::default();  
        }
        None => {
            let unit_direction = r.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t                 
        }
    }
}