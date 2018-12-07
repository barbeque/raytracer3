use cgmath::*;
use rand::{thread_rng, Rng};

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = thread_rng();

    let mut p : Vector3<f32>;

    loop {
         p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vector3::new(1.0, 1.0, 1.0);
         if p.magnitude2() < 1.0 {
             break;
         }
     }

     p
}


pub fn mul_vec(v1: &Vector3<f32>, v2: &Vector3<f32>) -> Vector3<f32> {
    // From the book, but I'm not sure what operation this actually is
    Vector3::<f32>::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

#[cfg(test)]
mod test_utils {
    use cgmath::*;
    use crate::utils::*;
    #[test]
    pub fn test_multiply_vectors() {
        let v1 = Vector3::<f32>::new(1.0, 3.0, 2.0);
        let v2 = Vector3::<f32>::new(2.0, 5.0, 1.5);
        let result = mul_vec(&v1, &v2);

        assert_eq!(result.x, 1.0 * 2.0);
        assert_eq!(result.y, 3.0 * 5.0);
        assert_eq!(result.z, 2.0 * 1.5);
    }

    #[test]
    pub fn test_random_in_unit_sphere() {
        for _sample in 0..1000 {
            // Not really a great way to test random...
            let choice = random_in_unit_sphere();
            assert!(choice.magnitude2() < 1.0);
        }
    }
}
