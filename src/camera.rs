use cgmath::*;
use ray::*;

pub struct Camera {
    pub lower_left_corner : Vector3<f32>,
    pub horizontal : Vector3<f32>,
    pub vertical : Vector3<f32>,
    pub origin : Vector3<f32>
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
