mod util;

use util::Vec3;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

fn main() -> std::io::Result<()>{
    let nx = 200;
    let ny = 100;

    let file = File::create("test.ppm")?;
    let mut file = LineWriter::new(file);
    file.write_all(b"P3\r\n")?;
    file.write_all(format!("{} {}\r\n", nx, ny).as_bytes())?;
    file.write_all(b"255\r\n")?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2 as f32;
            let mut col = Vec3::new(r,g,b);
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