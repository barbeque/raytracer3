use ray::*;
use cgmath::*;

pub struct HitRecord {
    pub t : f32,
    pub p : Vector3<f32>,
    pub normal : Vector3<f32>
}

impl HitRecord {
    pub fn new() -> HitRecord {
        return HitRecord {
            t: 0.0,
            p: Vector3::<f32>::new(0.0, 0.0, 0.0),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0)
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool;
}

pub struct Sphere {
    radius : f32,
    centre : Vector3<f32>
}

impl Sphere {
    pub fn new(centre : Vector3<f32>, radius : f32) -> Sphere {
        return Sphere {
            centre: centre,
            radius: radius
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
                return true;
            }
            let temp = (-b + (b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.centre) / self.radius;
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
            rec.t = temp_rec.t;
            rec.p = temp_rec.p;
            rec.normal = temp_rec.normal; // FIXME: bulky assignment
        }
    }

    return hit_anything;
}
