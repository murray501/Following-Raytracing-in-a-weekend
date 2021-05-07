mod myvec;
mod ray;
mod hitable;
mod camera;

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use ray::Ray;
use myvec::Vec3;
use hitable::{HitableList, Sphere, color};
use camera::Camera;
use rand::Rng;

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
    let sphere1 = Sphere{ center: Vec3::new(0.,0.,-1.), radius: 0.5};
    let sphere2 = Sphere{ center: Vec3::new(0.,-100.5,-1.), radius: 100.};
    hitables.list.push(Box::new(sphere1));
    hitables.list.push(Box::new(sphere2));    

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
                col += color(&r, &hitables);                
            }
            col /= ns as f32;
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
