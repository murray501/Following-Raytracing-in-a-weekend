mod myvec;
mod ray;

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use ray::Ray;
use myvec::Vec3;

fn main() -> std::io::Result<()>{
    let nx = 200;
    let ny = 100;

    let file = File::create("test.ppm")?;
    let mut file = LineWriter::new(file);
    file.write_all(b"P3\r\n")?;
    file.write_all(format!("{} {}\r\n", nx, ny).as_bytes())?;
    file.write_all(b"255\r\n")?;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let direction = lower_left_corner + horizontal * u + vertical * v;
            let r = Ray::new(origin, direction);
            let color = r.color() * 255.99;
            let ir = color.x.trunc() as u8;
            let ig = color.y.trunc() as u8;
            let ib = color.z.trunc() as u8;
            file.write_all(format!("{} {} {}\r\n", ir, ig, ib).as_bytes());
        }
    }
    file.flush()?;
    Ok(())
}