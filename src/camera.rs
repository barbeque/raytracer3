use cgmath::*;
use ray::*;

pub struct Camera {
    pub lower_left_corner : Vector3<f32>,
    pub horizontal : Vector3<f32>,
    pub vertical : Vector3<f32>,
    pub origin : Vector3<f32>
}

impl Camera {
    pub fn new(vertical_fov: f32, aspect_ratio: f32) -> Camera {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        Camera {
            lower_left_corner: Vector3::new(-half_width, -half_height, -1.0),
            horizontal: Vector3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0 * half_height, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
