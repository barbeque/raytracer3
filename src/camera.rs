use cgmath::*;
use ray::*;

pub struct Camera {
    pub lower_left_corner : Vector3<f32>,
    pub horizontal : Vector3<f32>,
    pub vertical : Vector3<f32>,
    pub origin : Vector3<f32>
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, up: Vector3<f32>, vertical_fov: f32, aspect_ratio: f32) -> Camera {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from
        }
    }

    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
