use cgmath::{ Vector3, InnerSpace };

pub struct Ray {
    pub origin : Vector3<f32>,
    pub direction : Vector3<f32>,
}

impl Ray {
    pub fn new(a : Vector3<f32>, b : Vector3<f32>) -> Ray {
        Ray { origin : a, direction : b.normalize() }
    }

    pub fn point_at_parameter(&self, t : f32) -> Vector3<f32> {
        return self.origin + (t * self.direction);
    }
}

#[cfg(test)]
mod ray_tests {
    use crate::ray::Ray;
    use cgmath::{ Vector3, InnerSpace };

    #[test]
    pub fn direction_is_always_normalized() {
        let r = Ray::new(Vector3::<f32>::new(140.0, 36.0, -2.0), Vector3::<f32>::new(17.0, 18.0, 19.0));
        assert_eq!(r.direction.magnitude(), 1.0);
    }
}
