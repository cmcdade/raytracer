use crate::vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    orig: vec3::Point3,
    dir: vec3::Vec3,
}

impl Ray {
    pub fn new(orig: vec3::Point3, dir: vec3::Vec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir,
        }
    }

    pub fn origin(self) -> vec3::Point3 {
        self.orig
    }

    pub fn direction(self) -> vec3::Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> vec3::Vec3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod test {
    use crate::ray::Ray;
    use crate::vec3;

    #[test]
    fn test_creation() {
        let o = vec3::Point3::new(0.0, 0.0, 0.0);
        let d = vec3::Vec3::new(0.0, 0.0, 0.0);
        let r = Ray::new(o, d);

        assert!(r.origin().x() == 0.0);
        assert!(r.origin().y() == 0.0);
        assert!(r.origin().z() == 0.0);

        assert!(r.direction().x() == 0.0);
        assert!(r.direction().y() == 0.0);
        assert!(r.direction().z() == 0.0);
    }
}
