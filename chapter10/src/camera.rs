use crate::myvec::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Vec3::new(0., 0., 0.),
            lower_left_corner: Vec3::new(-2., -1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., 2., 0.),
        }
    }
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * std::f32::consts::PI;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let lower_left_corner = origin - u* half_width - v * half_height - w;
        let horizontal = u * (half_width * 2.0);
        let vertical = v * (half_height * 2.0);
        
        Self {
            origin, lower_left_corner, horizontal, vertical
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin;
        return Ray::new(self.origin, direction);
    }
}