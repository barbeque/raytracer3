extern crate cgmath;
use cgmath::*;

mod ray;
use ray::{Ray};

fn hit_sphere(centre : Vector3<f32>, radius : f32, ray : &Ray) -> bool {
    let oc = ray.origin - centre;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    (discriminant > 0.0)
}

pub fn lerp_v(t : f32, start : Vector3<f32>, end : Vector3<f32>) -> Vector3<f32> {
    assert!(t <= 1.0);
    assert!(t >= 0.0);

    (1.0 - t) * start + t * end
}

fn colour(r: &Ray) -> Vector3<f32> {
    if hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vector3::new(1.0, 0.0, 0.0) // pixels of the sphere
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    // lerp blue depending on y-coordinate
    lerp_v(t, Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.5, 0.7, 1.0))
}

fn main() {
    let nx : i32 = 200;
    let ny : i32 = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = colour(&r);

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
