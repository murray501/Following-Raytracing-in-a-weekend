use crate::myvec::Vec3;
use crate::ray::Ray;
use rand::{Rng, random};

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0.0..1.0) as f32;
        let y = rng.gen_range(0.0..1.0) as f32;
        let p = Vec3::new(x, y, 0.0) * 2.0 - Vec3::new(1., 1., 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture * 0.5;
        let theta = vfov * std::f32::consts::PI/ 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let lower_left_corner = origin - u* half_width*focus_dist - v * half_height*focus_dist - w * focus_dist;
        let horizontal = u * (half_width * 2.0 * focus_dist);
        let vertical = v * (half_height * 2.0 * focus_dist);
        
        Self {
            origin, lower_left_corner, horizontal, vertical, lens_radius, u, v, w
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y; 
        let direction = self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset;
        return Ray::new(self.origin + offset, direction);
    }
}