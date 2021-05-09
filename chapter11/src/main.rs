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
use hitable::{HitableList, Sphere, color, random_scene};
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use rand::{Rng, random};
use std::rc::Rc;

fn main() -> std::io::Result<()>{
    let nx = 200;
    let ny = 100;
    let ns = 10;

    let file = File::create("test2.ppm")?;
    let mut file = LineWriter::new(file);
    file.write_all(b"P3\r\n")?;
    file.write_all(format!("{} {}\r\n", nx, ny).as_bytes())?;
    file.write_all(b"255\r\n")?;
 
    let hitables = random_scene();
   
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom, lookat, vup, 20.0, nx as f32 / ny as f32, aperture, dist_to_focus);
    
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
