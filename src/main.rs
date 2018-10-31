extern crate cgmath;
use cgmath::*;
extern crate rand;
use rand::{ thread_rng, Rng };
#[macro_use]
extern crate clap;
use clap::{Arg, App};
extern crate image;
use std::time::{ Instant };
extern crate rayon;
use rayon::prelude::*;

mod ray;
use ray::{Ray};
mod hittables;
use hittables::*;
mod camera;
use camera::{ Camera };
mod utils;
use utils::*;
mod materials;
mod generators;
use generators::*;

pub fn lerp_v(t : f32, start : Vector3<f32>, end : Vector3<f32>) -> Vector3<f32> {
    assert!(t <= 1.0);
    assert!(t >= 0.0);

    (1.0 - t) * start + t * end
}

fn colour(r: &Ray, world : &Vec<Box<Hittable>>, depth: i32) -> Vector3<f32> {
    let mut rec : HitRecord = HitRecord::new();
    if hit_visitor(world, r, 0.001, std::f32::MAX, &mut rec) {
        let mut scattered = Ray::new(Vector3::<f32>::new(0.0, 0.0, 0.0), Vector3::<f32>::new(0.0, 0.0, 0.0));
        let mut attenuation = Vector3::<f32>::new(0.0, 0.0, 0.0);

        if depth < 50 && rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            let result = colour(&scattered, &world, depth + 1);
            return mul_vec(&attenuation, &result); // not sure if this is right
        }
        else {
            return Vector3::<f32>::new(0.0, 0.0, 0.0);
        }
    }
    else {
        // nothing hit, draw the 'sky'
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return lerp_v(t, Vector3::<f32>::new(1.0, 1.0, 1.0), Vector3::<f32>::new(0.5, 0.7, 1.0));
    }
}

fn main() {
    let m = App::new("raytracer3")
            .about("Overcomplicated path-tracer")
            .arg(Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true))
            .arg(Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true))
            .arg(Arg::with_name("samples")
                .short("s")
                .long("samples")
                .takes_value(true))
            .arg(Arg::with_name("aperture")
                .short("a")
                .long("aperture")
                .takes_value(true))
            .arg(Arg::with_name("OUTPUT")
                .help("Sets the image file to output")
                .required(false)
                .index(1))
            .get_matches();

    let nx = value_t!(m, "width", u32).unwrap_or(200);
    let ny = value_t!(m, "height", u32).unwrap_or(100);
    let number_of_samples = value_t!(m, "samples", u32).unwrap_or(100);

    let look_from = Vector3::new(3.0, 2.0, 2.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let look_up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let aperture = value_t!(m, "aperture", f32).unwrap_or(0.75);

    let cam = Camera::new(look_from, look_at, look_up, 90.0, nx as f32 / ny as f32, aperture, dist_to_focus);

    let world = random_scene();
    let output_filename = m.value_of("OUTPUT").unwrap_or("image.png");

    let mut image_buf = Vec::<u8>::with_capacity((nx * ny * 3) as usize);
    image_buf.resize((nx * ny * 3) as usize, 0);
    assert_eq!(image_buf.len(), (nx * ny * 3) as usize);

    let start = Instant::now();

    for j in 0..ny {
        for i in 0..nx {
            // Do multisamples with Rayon
            let mut col : Vector3<f32> = (0..number_of_samples).into_par_iter()
                .map(|_sample| {
                    let mut rng = thread_rng();

                    let u = (i as f64 + rng.gen::<f64>()) as f32 / nx as f32;
                    let v = (j as f64 + rng.gen::<f64>()) as f32 / ny as f32;
                    let ray = cam.get_ray(u, v);
                    colour(&ray, &world, 0)
                })
                .sum();

            col /= number_of_samples as f32;
            // gamma correction
            col = Vector3::new( col.x.sqrt(), col.y.sqrt(), col.z.sqrt() );

            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            let idx = (((ny - 1) - j) * nx * 3 + i * 3) as usize;

            image_buf[idx] = ir;
            image_buf[idx + 1] = ig;
            image_buf[idx + 2] = ib;
        }
    }

    println!("Completed in {} seconds", start.elapsed().as_secs());

    // write image out
    image::save_buffer(output_filename, &image_buf, nx, ny, image::RGB(8)).unwrap();
}

#[cfg(test)]
mod lerp_tests {
    use cgmath::*;
    use lerp_v;

    #[test]
    pub fn works_at_extents() {
        let black = Vector3::new(0.0, 0.0, 0.0);
        let white = Vector3::new(1.0, 1.0, 1.0);

        assert_eq!(lerp_v(0.0, black, white), Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(lerp_v(0.5, black, white), Vector3::new(0.5, 0.5, 0.5));
        assert_eq!(lerp_v(1.0, black, white), Vector3::new(1.0, 1.0, 1.0));
    }
}
