use ray::*;
use cgmath::*;
use materials::{Material, Lambertian};

#[derive(Clone)]
pub struct HitRecord {
    pub t : f32,
    pub p : Vector3<f32>,
    pub normal : Vector3<f32>,
    pub material : Box<Material>
}

impl HitRecord {
    pub fn new() -> HitRecord {
        return HitRecord {
            t: 0.0,
            p: Vector3::<f32>::new(0.0, 0.0, 0.0),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0),
            material: Box::new(Lambertian::new(0.0, 0.0, 0.0))
        };
    }
}

pub trait Hittable : Sync {
    fn hit(&self, r: &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool;
}

pub struct Sphere {
    radius : f32,
    centre : Vector3<f32>,
    material : Box<Material>
}

impl Sphere {
    pub fn new(centre : Vector3<f32>, radius : f32, material: Box<Material>) -> Sphere {
        return Sphere {
            centre: centre,
            radius: radius,
            material: material
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool {
        let oc = r.origin - self.centre;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.centre) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.centre) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
        }
        false
    }
}

pub fn hit_visitor(v: &Vec<Box<Hittable>>, r: &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for item in v {
        if item.hit(&r, t_min, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *rec = temp_rec.clone(); // oh shit material wasn't copied here
        }
    }

    return hit_anything;
}
