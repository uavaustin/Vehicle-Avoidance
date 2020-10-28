use super::vector::Vector;
use std::ops::Mul;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    real: f32,
    i: f32,
    j: f32,
    k: f32,
}
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        let w2: f32 = rhs.get_real();
        let i2: f32 = rhs.get_i();
        let j2: f32 = rhs.get_j();
        let k2: f32 = rhs.get_k();
        let w1: f32 = self.real;
        let i1: f32 = self.i;
        let j1: f32 = self.j;
        let k1: f32 = self.k;

        let new_w: f32 = w1 * w2 - i1 * i2 - j1 * j2 - k1 * k2;
        let new_i: f32 = w1 * i2 + i1 * w2 + j1 * k2 - k1 * j2;
        let new_j: f32 = w1 * j2 + j1 * w2 + k1 * i2 - i1 * k2;
        let new_k: f32 = w1 * k2 + k1 * w2 + i1 * j2 - j1 * i2;

        Quaternion::new(new_w, Vector::new(new_i, new_j, new_k))
    }
}

impl Quaternion {
    pub fn new(real: f32, imaginary: Vector) -> Self {
        Self {
            real: real,
            i: imaginary.get_i(),
            j: imaginary.get_j(),
            k: imaginary.get_k(),
        }
    }

    pub fn rotation(angle: f32, dir: Vector) -> Self {
        let dir_vector = dir.to_dir() * angle.sin();
        Self {
            real: angle.cos(),
            i: dir_vector.get_i(),
            j: dir_vector.get_j(),
            k: dir_vector.get_k(),
        }
    }

    pub fn get_real(&self) -> f32 {
        self.real
    }

    pub fn get_i(&self) -> f32 {
        self.i
    }

    pub fn get_j(&self) -> f32 {
        self.j
    }

    pub fn get_k(&self) -> f32 {
        self.k
    }

    pub fn get_imaginary(&self) -> Vector {
        Vector::new(self.i, self.j, self.k)
    }

    pub fn get_conjugate(&self) -> Quaternion {
        Quaternion::new(self.real, self.get_imaginary() * -1f32)
    }
}
