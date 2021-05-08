mod myvec;
mod ray;
mod hitable;
mod camera;
mod material;

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use ray::Ray;
use myvec::Vec3;
use hitable::{HitableList, Sphere, color};
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use rand::Rng;
use std::rc::Rc;

fn main() -> std::io::Result<()>{
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let file = File::create("test.ppm")?;
    let mut file = LineWriter::new(file);
    file.write_all(b"P3\r\n")?;
    file.write_all(format!("{} {}\r\n", nx, ny).as_bytes())?;
    file.write_all(b"255\r\n")?;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut hitables = HitableList::default();
    let sphere1 = Sphere::new(Vec3::new(0.,0.,-1.), 0.5,
        Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))));

    let sphere2 = Sphere::new(Vec3::new(0.,-100.5,-1.),100.0,
    Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))));
    
    let sphere3 = Sphere::new(Vec3::new(1.,0.,-1.), 0.5,
    Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)));

    let sphere4 = Sphere::new(Vec3::new(-1.,0.,-1.),0.5,
Rc::new(Dielectric::new(1.5)));
    
    hitables.list.push(Box::new(sphere1));
    hitables.list.push(Box::new(sphere2));  
    hitables.list.push(Box::new(sphere3));    
    hitables.list.push(Box::new(sphere4));      

    let camera = Camera::default();
    let mut rng = rand::thread_rng();
    
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for s in 0..ns {
                let random1 = rng.gen_range(0.0..1.0) as f32;
                let random2 = rng.gen_range(0.0..1.0) as f32;
                let u = (i as f32 + random1) / nx as f32;
                let v = (j as f32 + random2) / ny as f32;
                let r = camera.get_ray(u, v);
                let p = r.point_at_paramter(2.0);
                col += color(&r, &hitables,0);                
            }
            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            col *= 255.99;
            let ir = col.x.trunc() as u8;
            let ig = col.y.trunc() as u8;
            let ib = col.z.trunc() as u8;
            file.write_all(format!("{} {} {}\r\n", ir, ig, ib).as_bytes());
        }
    }

    file.flush()?;
    Ok(())
}
