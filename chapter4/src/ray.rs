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
        if (self.hit_sphere(Vec3::new(0., 0., -1.), 0.5)) {
            return Vec3::new(1., 0., 0.);
        }
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t 
    }

    pub fn hit_sphere(&self, center: Vec3, radius: f32) -> bool {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        return (discriminant > 0.0);
    }
}



