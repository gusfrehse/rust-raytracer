use std::ops;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        return Vec3 { e: [0.0, 0.0, 0.0] };
    }

    pub fn new<T, U, V>(x: T, y: U, z: V) -> Vec3
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        Vec3 {
            e: [x.into(), y.into(), z.into()],
        }
    }

    pub fn length2(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length2().sqrt()
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    pub fn unit(self) -> Vec3 {
        self * (1.0 / self.length())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - _rhs.e[0],
                self.e[1] - _rhs.e[1],
                self.e[2] - _rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            e: [_rhs * self.e[0], _rhs * self.e[1], _rhs * self.e[2]],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [_rhs.e[0] * self, _rhs.e[1] * self, _rhs.e[2] * self],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / _rhs, self.e[1] / _rhs, self.e[2] / _rhs],
        }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self / _rhs.e[0], self / _rhs.e[1], self / _rhs.e[2]],
        }
    }
}
