use crate::myvec::Vec3;
use crate::ray::Ray;
#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_paramter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_paramter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}

#[derive(Default)]
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in self.list.iter() {
            if hitable.hit(r,t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        return hit_anything;
    }    
}
    
pub fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, std::f32::MAX, &mut rec){
        return (rec.normal + Vec3::new(1.,1.,1.)) * 0.5;
    } else {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t 
    }
}