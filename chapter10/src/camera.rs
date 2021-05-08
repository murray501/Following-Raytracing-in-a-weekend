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
    pub fn new(vfov: f32, aspect: f32) -> Self {
        let theta = vfov * std::f32::consts::PI;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        let lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        let horizontal = Vec3::new(2.0*half_width, 0., 0.);
        let vertical = Vec3::new(0., 2.0 * half_height, 0.0);
        let origin = Vec3::new(0., 0., 0.);
        Self {
            origin, lower_left_corner, horizontal, vertical
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin;
        return Ray::new(self.origin, direction);
    }
}