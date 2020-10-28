use super::vector::Vector;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    real: f32,
    imaginary: Vector,
}
