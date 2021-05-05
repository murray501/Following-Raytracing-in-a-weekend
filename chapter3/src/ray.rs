use crate::myvec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {origin, direction}
    }
    
    pub fn point_at_paramter(&self, parameter: f32) -> Vec3 {
        self.origin + self.direction * parameter
    }

    pub fn color(&self) -> Vec3 {
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t 
    }
}



