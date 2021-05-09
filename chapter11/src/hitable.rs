use crate::myvec::Vec3;
use crate::ray::Ray;
use crate::material::{Material, Lambertian, Metal, Dielectric};
use std::rc::Rc;
use rand::Rng;
use rand::rngs::ThreadRng;

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

impl HitableList {
    fn add(&mut self, hitable: Box<dyn Hitable>) {
        self.list.push(hitable);
    }

    fn length(&self) -> usize {
        self.list.len()
    }
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

pub fn random_scene() -> HitableList {
    let mut list = HitableList::default();
    let sphere = Sphere::new(Vec3::new(0., -1000., 0.), 1000.,
            Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))));
    list.add(Box::new(sphere));
    
    let mut rng = rand::thread_rng();
    
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(a as f32 + 0.9 * drand(&mut rng), 0.2, b as f32 + 0.9 * drand(&mut rng));
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let random = drand(&mut rng);
                let hitable = 
                    if random < 0.8 {
                        let r = drand(&mut rng) * drand(&mut rng);
                        let g = drand(&mut rng) * drand(&mut rng);
                        let b = drand(&mut rng) * drand(&mut rng);
                        Sphere::new(center, 0.2,
                        Rc::new(Lambertian::new(Vec3::new(r, g, b))))
                    } else if random < 0.95 {
                        let x = 0.5 * (1.0 + drand(&mut rng));
                        let y = 0.5 * (1.0 + drand(&mut rng));
                        let z = 0.5 * (1.0 + drand(&mut rng));
                        let fuzz = 0.5 * drand(&mut rng);
                        Sphere::new(center, 0.2,
                            Rc::new(Metal::new(Vec3::new(x, y, z), fuzz)))
                    } else {
                        Sphere::new(center, 0.2, 
                            Rc::new(Dielectric::new(1.5)))
                    };
                list.add(Box::new(hitable));
            }
        }
    }
    
    list.add(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1.0, 
                Rc::new(Dielectric::new(1.5)))));

    list.add(Box::new(Sphere::new(Vec3::new(-4., -1., 0.), 1.0,
                    Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));

    list.add(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1.0,  
                    Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));            
    
    return list;
}

fn drand(rng: &mut ThreadRng) -> f32 {
    rng.gen_range(0.0..1.0) as f32
}