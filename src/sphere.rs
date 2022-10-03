use crate::hittable;
use crate::ray;
use crate::vec3;

pub struct Sphere {
    center: vec3::Point3,
    radius: f64,
}

impl Sphere {
    fn new(cen: vec3::Point3, r: f64) -> Self {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl hittable::Hit for Sphere {
    fn hit(&self, r: ray::Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_discriminant = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrt_discriminant) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}
