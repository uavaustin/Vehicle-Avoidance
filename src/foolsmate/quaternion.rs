use super::vector::Vector;
use super::point::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Quaternion {
    dir:Vector,
    real:f32,

}

impl Quaternion {
    pub fn new(rotation:f32, dir:Vector) -> Self{
        dir:rotation.sin() * dir,
        real: rotation.cos(),
    }

    pub fn define(real:f32, dir:Vector) -> Self {
        real:real,
        dir:im,
    }

    pub fn get_real(&self) -> f32 {
        self.real
    }

    pub fn get_dir(&self) -> Vector {
        self.dir
    }

    pub fn get_rotation(&self) -> Vector {
        self.rotation
    }

    pub fn rotate(&self, v:Vector) -> Vector{
        let temp:Quaternion = Quaternion::define(0f32, v);
        let intermediate:Quaternion = self.multiply(temp);
        let result:Quaternion = intermediate.multiply(self.inverse());
        result.get_dir();
    }

    pub fn inverse(&self, v:Vector) -> Quaternion{
        Quaternion::define(self.get_real(), -1f32 * self.get_dir() )
    }

    pub fn multiply(&self, q:Quaternion) -> Quaternion {
        let w1:f32 = self.real;
        let w2:f32 = q.get_real();
        let x1:f32 = self.dir.get_x();
        let x2:f32 = q.get_dir().get_x();
        let y1:f32 = self.dir.get_y();
        let y2:f32 = q.get_dir().get_y();
        let z1:f32 = self.dir.get_z();
        let z2:f32 = q.get_dir().get_z();

        let new_real:f32 = w1 * w2 - x1 * x2 - y1*y2 - z1*z2;
        let new_i:f32 = w1*x2 + x1*w2 + y1*z2 - z1*y2;
        let new_j:f32 = w1 * y2 + y1 * w2 + z1 * x2 - x1 * z2;
        let new_k:f32 = w1 * z2 + z1 * w2 + z1 * y2 - y1 * x

        let new_vector:Vector = Vector::new(new_i, new_j, new_k);

        Quaternion::define(new_real, new_vector)
    }
}

// TODO: Tests
