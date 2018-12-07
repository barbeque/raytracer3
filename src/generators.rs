use rand::{thread_rng, Rng};
use crate::hittables::*;
use cgmath::*;
use crate::materials::*;

pub fn random_scene() -> Vec<Box<Hittable>> {
    let mut rng = thread_rng();

    let mut list = Vec::<Box<Hittable>>::new();
    list.push(Box::new(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(0.5, 0.5, 0.5)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat : f32 = rng.gen();
            let x = (a as f32) + 0.9 * rng.gen::<f32>();
            let z = (b as f32) + 0.9 * rng.gen::<f32>();
            let centre = Vector3::new(x, 0.2, z);
            if (centre - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.9 {
                    // diffuse
                    list.push(Box::new(
                        Sphere::new(
                            centre, 0.2,
                            Box::new(
                                Lambertian::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>())
                            )
                        )
                    ));
                }
                else if choose_mat < 0.95 {
                    // metal
                    list.push(Box::new(
                        Sphere::new(
                            centre, 0.2,
                            Box::new(
                                Metal::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), rng.gen::<f32>())
                            )
                        )
                    ));
                }
                else {
                    // glass
                    list.push(Box::new(
                        Sphere::new(
                            centre, 0.2,
                            Box::new(
                                Dielectric::new(1.5)
                            )
                        )
                    ));
                }
            }
        }
    }

    // add in the big spheres
    list.push(Box::new(
        Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0,
            Box::new(Dielectric::new(1.5)))
    ));
    list.push(Box::new(
        Sphere::new(Vector3::new(-2.0, 1.0, 0.0), 1.0,
            Box::new(Lambertian::new(0.4, 0.2, 0.1)))
    ));
    list.push(Box::new(
        Sphere::new(Vector3::new(2.0, 1.0, 0.0), 1.0,
            Box::new(Metal::new(0.7, 0.6, 0.5, 0.0)))
    ));

    list
}
