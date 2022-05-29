use image::{ImageBuffer, Pixel, RgbImage};
use rand::prelude::*;

mod vec3;
mod ray;

use vec3::*;
use ray::*;

fn main() {
    // image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let width: u32 = 400;
    let height: u32 = (width as f64 / aspect_ratio).floor() as u32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0, 0, 0);
    let horizontal = Vec3::new(viewport_width, 0, 0);
    let vertical = Vec3::new(0, viewport_height, 0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0, 0, focal_length);

    let mut img : RgbImage = ImageBuffer::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let u = i as f64 / (width as f64 - 1.0);
            let v = j as f64 / (height as f64 - 1.0);

            let r = Ray {
                orig : origin,
                dir : lower_left_corner + u * horizontal + (-v) * vertical - origin,
            };

            let unit_dir = r.direction().unit();
            let t = 0.5 * (unit_dir.e[1] + 1.0);
            let color = (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);

            write_pixel(&mut img, i, j, color);
        }

    }

    img.save("test.png").unwrap();
}

fn write_pixel<U>(img : &mut image::ImageBuffer<image::Rgb<u8>, U>, x : u32, y : u32, c : Color)
where
    U : std::ops::Deref<Target = [u8]> + std::ops::DerefMut // rust is simple..
{
    let p = image::Rgb([(c.e[0] * 255.999).floor() as u8,
                        (c.e[1] * 255.999).floor() as u8,
                        (c.e[2] * 255.999).floor() as u8]);
    img.put_pixel(x, y, p);
}
