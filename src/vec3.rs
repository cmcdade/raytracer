use std::fmt;
use std::ops;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f64 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
}

#[inline(always)]
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    (v.e[0] * u.e[0]) + (v.e[1] * u.e[1]) + (v.e[2] * u.e[2])
}

#[inline(always)]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        (u.y() * v.z()) - (u.z() * v.y()),
        (u.z() * v.x()) - (u.x() * v.z()),
        (u.x() * v.y()) - (u.y() * v.x()),
    )
}

#[inline(always)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(self: &mut Vec3, other: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[2],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(self: &mut Vec3, t: f64) {
        *self = Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(self: &mut Vec3, t: f64) {
        *self *= 1.0 / t
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.e[0] as i32, self.e[1] as i32, self.e[2] as i32
        )
    }
}

#[cfg(test)]
mod test {
    use crate::vec3;
    use crate::vec3::Vec3;

    #[test]
    fn test_vector_creation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }
    #[test]
    fn test_vector_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_sum = v1 + v2;
        assert_eq!(v_sum.x(), 5.0);
        assert_eq!(v_sum.y(), 7.0);
        assert_eq!(v_sum.z(), 9.0);
    }
    #[test]
    fn test_vector_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_diff = v2 - v1;
        assert_eq!(v_diff.x(), 3.0);
        assert_eq!(v_diff.y(), 3.0);
        assert_eq!(v_diff.z(), 3.0);
    }

    #[test]
    fn test_vector_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_mut = v2 * v1;
        assert_eq!(v_mut.x(), 4.0);
        assert_eq!(v_mut.y(), 10.0);
        assert_eq!(v_mut.z(), 18.0);
    }

    #[test]
    fn test_vector_div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_diff = v2 - v1;
        assert_eq!(v_diff.x(), 3.0);
        assert_eq!(v_diff.y(), 3.0);
        assert_eq!(v_diff.z(), 3.0);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_dot = vec3::dot(v1, v2);

        assert_eq!(v_dot, 32.0);
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_cross = vec3::cross(v1, v2);

        assert_eq!(v_cross.x(), -3.0);
        assert_eq!(v_cross.y(), 6.0);
        assert_eq!(v_cross.z(), -3.0);
    }
    #[test]
    fn test_vector_length() {
        let v1 = Vec3::new(1.0, 2.0, 2.0);
        let v_length = v1.length();

        assert_eq!(v_length, 3.0);
    }

    #[test]
    fn test_vector_length_sqr() {
        let v1 = Vec3::new(1.0, 2.0, 2.0);
        let length_sqr = v1.length_squared();

        assert_eq!(length_sqr, 9.0);
    }

    #[test]
    fn test_unit_vector() {
        let v1 = Vec3::new(1.0, 2.0, 2.0);
        let v_unit = vec3::unit_vector(v1);

        assert_eq!(v_unit.x(), -3.0);
        assert_eq!(v_unit.y(), 6.0);
        assert_eq!(v_unit.z(), -3.0);
    }
}
