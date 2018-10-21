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

pub fn lerp_v(t : f32, start : Vector3<f32>, end : Vector3<f32>) -> Vector3<f32> {
    assert!(t <= 1.0);
    assert!(t >= 0.0);

    (1.0 - t) * start + t * end
}

fn colour(r: &Ray, world : &Vec<Box<Hittable>>) -> Vector3<f32> {
    let mut rec : HitRecord = HitRecord::new();
    if hit_visitor(world, r, 0.0, std::f32::MAX, &mut rec) {
        // Reflect off a diffuse thing
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * colour( &Ray::new(rec.p, target - rec.p), world);
    }
    else {
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
    let s1 = Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    let s2 = Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));
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
                col += colour(&ray, &world);
            }

            col /= number_of_samples as f32;

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
