extern crate cgmath;
use cgmath::*;
extern crate rand;
use rand::{ thread_rng, Rng };
#[macro_use]
extern crate clap;
use clap::{Arg, App};

mod ray;
use ray::{Ray};
mod hittables;
use hittables::*;
mod camera;
use camera::{ Camera };
mod utils;
use utils::*;
mod materials;
use materials::*;

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
            .get_matches();

    let nx = value_t!(m, "width", u32).unwrap_or(200);
    let ny = value_t!(m, "height", u32).unwrap_or(100);
    let number_of_samples = value_t!(m, "samples", u32).unwrap_or(100);
    let mut rng = thread_rng();
    let cam = Camera::new(90.0, nx as f32 / ny as f32);
    let R = std::f32::consts::PI / 4.0;

    println!("P3\n{} {}\n255", nx, ny);

    let mut world = Vec::<Box<Hittable>>::new();
    /*let s1 = Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(0.8, 0.3, 0.3))));
    let s2 = Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(0.8, 0.8, 0.0))));
    let s3 = Box::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(0.8, 0.6, 0.2, 0.3))));
    let s4 = Box::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5))));
    let s5 = Box::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5))));
    world.push(s1);
    world.push(s2);
    world.push(s3);
    world.push(s4);
    world.push(s5);*/

    let s1 = Box::new(Sphere::new(Vector3::new(-R, 0.0, -1.0), R, Box::new(Lambertian::new(0.0, 0.0, 1.0))));
    let s2 = Box::new(Sphere::new(Vector3::new( R, 0.0, -1.0), R, Box::new(Lambertian::new(1.0, 0.0, 0.0))));
    world.push(s1);
    world.push(s2);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _sample in 0..number_of_samples {
                // Multisample mode
                let u = (i as f64 + rng.gen::<f64>()) as f32 / nx as f32;
                let v = (j as f64 + rng.gen::<f64>()) as f32 / ny as f32;
                let ray = cam.get_ray(u, v);
                col += colour(&ray, &world, 0);
            }

            col /= number_of_samples as f32;
            // gamma correction
            col = Vector3::new( col.x.sqrt(), col.y.sqrt(), col.z.sqrt() );

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
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
