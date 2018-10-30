use cgmath::*;
use ray::*;
use rand::{thread_rng, Rng};

pub struct Camera {
    pub lower_left_corner : Vector3<f32>,
    pub horizontal : Vector3<f32>,
    pub vertical : Vector3<f32>,
    pub origin : Vector3<f32>,
    pub lens_radius: f32,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>
}

fn random_in_unit_disc() -> Vector3<f32> {
    let mut p : Vector3<f32>;
    let mut rng = thread_rng();

    loop {
        p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vector3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, up: Vector3<f32>, vertical_fov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lens_radius: aperture / 2.0,
            lower_left_corner: look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            u: u,
            v: v,
            w: w
        }
    }

    pub fn get_ray(&self, s : f32, t : f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}
