use ray::*;
use hittables::{ HitRecord };
use utils::*;
use cgmath::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool;
    fn box_clone(&self) -> Box<Material>;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vector3<f32>
}

impl Lambertian {
    pub fn new(r: f32, g: f32, b: f32) -> Lambertian {
        Lambertian {
            albedo: Vector3::new(r, g, b)
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vector3<f32>,
    fuzziness: f32
}

impl Metal {
    pub fn new(r: f32, g: f32, b: f32, fuzziness: f32) -> Metal {
        Metal {
            albedo: Vector3::new(r, g, b),
            fuzziness: fuzziness.min(1.0)
        }
    }
}

fn reflect(v : &Vector3<f32>, n : &Vector3<f32>) -> Vector3<f32> {
    return v - 2.0 * v.dot(*n) * n;
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
        // the bigger the sphere, the fuzzier reflections will be
        let fuzzy_offshoot = self.fuzziness * random_in_unit_sphere();
        *scattered = Ray::new(rec.p, reflected + fuzzy_offshoot);
        *attenuation = self.albedo;

        scattered.direction.dot(rec.normal) > 0.0
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.box_clone()
    }
}

#[cfg(test)]
mod material_tests {
    use materials::{ Lambertian, Metal };

    #[test]
    pub fn test_box_clone_trait() {
        let a1 = Box::new(Lambertian::new(1.0, 0.8, 0.5));
        let a2 = a1.clone();
        assert_eq!(a2.albedo.x, 1.0);
        assert_eq!(a2.albedo.y, 0.8);
        assert_eq!(a2.albedo.z, 0.5);
    }

    #[test]
    pub fn test_lambertian_constructor() {
        let l = Lambertian::new(1.0, 2.0, 3.0);
        assert_eq!(l.albedo.x, 1.0);
        assert_eq!(l.albedo.y, 2.0);
        assert_eq!(l.albedo.z, 3.0);
    }

    #[test]
    pub fn test_metal_constructor() {
        let m = Metal::new(1.0, 2.0, 3.0, 0.5);
        assert_eq!(m.albedo.x, 1.0);
        assert_eq!(m.albedo.y, 2.0);
        assert_eq!(m.albedo.z, 3.0);
        assert_eq!(m.fuzziness, 0.5);
    }

    #[test]
    pub fn test_metal_constructor_fuzziness_clamps() {
        let m = Metal::new(1.0, 1.0, 1.0, 6.0);
        assert_eq!(m.fuzziness, 1.0); // must clamp to <= 1.0
    }
}
