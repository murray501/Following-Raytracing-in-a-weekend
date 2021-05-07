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
}



