extern crate ray;
extern crate image;

use image::{ImageBuffer, Luma, ImageLuma8};
use ray::objects::*;
use ray::math::*;
use ray::intersect::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let spheres = &[
        Sphere::from_radius(1.0_f64),
        Sphere::new(1.0_f64, Vec3::new(-3_f64, 0_f64, 0_f64)),
        Sphere::new(1.0_f64, Vec3::new(3_f64, 0_f64, 0_f64)),
    ];

    let ref mut fout = File::create(&Path::new("test.png")).unwrap();
    let mut img = ImageBuffer::new(640, 480);    

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ex = (x as f64 - 320_f64) * 0.5_f64;
        let ey = (y as f64 - 240_f64) * 0.5_f64;
        
        let r = Ray3::new(Vec3::new(0_f64, 0_f64, -5_f64),
                          Vec3::new(ex, ey, 100_f64));

        for (i, s) in spheres.iter().enumerate() {
            match s.intersect(&r) {
                Some(_) => {
                    *pixel = Luma([(50 as u8) + (i as u8) * (50 as u8)]);
                    break;
                },
                None => continue,
            }
        }
    }

    let _ = ImageLuma8(img).save(fout, image::PNG).unwrap();
}
