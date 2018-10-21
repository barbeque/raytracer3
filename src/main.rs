extern crate cgmath;
use cgmath::*;
extern crate rand;
use rand::{ thread_rng, Rng };

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
    let nx : i32 = 200;
    let ny : i32 = 100;
    let number_of_samples = 100;
    let mut rng = thread_rng();
    let cam = Camera::new();

    println!("P3\n{} {}\n255", nx, ny);

    let mut world = Vec::<Box<Hittable>>::new();
    let s1 = Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(0.8, 0.3, 0.3))));
    let s2 = Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(0.8, 0.8, 0.0))));
    let s3 = Box::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(0.8, 0.6, 0.2))));
    let s4 = Box::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(0.8, 0.8, 0.8))));
    world.push(s1);
    world.push(s2);
    world.push(s3);
    world.push(s4);

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
