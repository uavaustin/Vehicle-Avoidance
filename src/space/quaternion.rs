use super::vector::Vector;
use std::fmt;
use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    real: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl std::cmp::PartialEq for Quaternion {
    fn eq(&self, rhs: &Quaternion) -> bool {
        const ACCEPTABLE_MARGIN: f32 = 0.01f32;
        let real_eq: bool = (rhs.get_real() - self.get_real()) == 0f32
            || ((rhs.get_real() - self.get_real()) / self.get_real()) < ACCEPTABLE_MARGIN;
        let i_eq: bool = (rhs.get_imaginary().get_i() - self.get_imaginary().get_i()) == 0f32
            || ((rhs.get_imaginary().get_i() - self.get_imaginary().get_i())
                / self.get_imaginary().get_i())
                < ACCEPTABLE_MARGIN;

        let j_eq: bool = (rhs.get_imaginary().get_j() - self.get_imaginary().get_j()) == 0f32
            || ((rhs.get_imaginary().get_j() - self.get_imaginary().get_j())
                / self.get_imaginary().get_j())
                < ACCEPTABLE_MARGIN;
        let k_eq: bool = (rhs.get_imaginary().get_k() - self.get_imaginary().get_k()) == 0f32
            || ((rhs.get_imaginary().get_k() - self.get_imaginary().get_k())
                / self.get_imaginary().get_k())
                < ACCEPTABLE_MARGIN;

        real_eq && i_eq && j_eq && k_eq
    }
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

    //Rotation angle should be half the desired total rotation!!
    pub fn rotation(angle: f32, dir: Vector) -> Self {
        let dir_vector = dir.to_dir() * angle.sin();
        Self {
            real: angle.cos(),
            i: dir_vector.get_i(),
            j: dir_vector.get_j(),
            k: dir_vector.get_k(),
        }
    }

    pub fn rotate_vector(&self, v1: Vector) -> Vector {
        let temp: Quaternion = Quaternion::new(0f32, v1);
        let rotation: Quaternion = Quaternion::new(self.real, self.get_imaginary());
        (rotation * temp * rotation.get_conjugate()).get_imaginary()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_conjugate() {
        let v1: Vector = Vector::new(2f32, 1f32, 3f32);
        let real: f32 = 4f32;
        let q1: Quaternion = Quaternion::new(real, v1);
        let q2: Quaternion = q1.get_conjugate();
        assert_eq!(q1.get_real(), q2.get_real());
        assert_eq!(q1.get_imaginary(), q2.get_imaginary() * -1f32);
        assert_eq!((q1 * q2).get_imaginary(), Vector::new(0f32, 0f32, 0f32));
    }

    #[test]
    pub fn test_rotation_simple() {
        let to_rotate: Vector = Vector::new(0f32, 1f32, 0f32);
        let axis: Vector = Vector::new(1f32, 0f32, 0f32);
        let angle: f32 = std::f32::consts::PI / 4f32;
        let rotation: Quaternion = Quaternion::rotation(angle, axis);
        let test: Vector = Vector::new(0f32, 0f32, 1f32);
        assert_eq!((rotation.rotate_vector(to_rotate)), test);
    }

    #[test]
    pub fn test_rotation() {
        let to_rotate: Vector = Vector::new(0f32, 0f32, 2f32);
        let axis: Vector = Vector::new(1f32, 1f32, 1f32);
        let angle: f32 = std::f32::consts::PI / 3f32;
        let rotation: Quaternion = Quaternion::rotation(angle, axis);
        let test: Vector = Vector::new(2f32, 0f32, 0f32);
        assert_eq!(rotation.rotate_vector(to_rotate), test);
    }
}
