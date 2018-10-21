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
