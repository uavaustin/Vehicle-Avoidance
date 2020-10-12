use std::f32;
#[derive(Clone, Debug, PartialEq)]
pub struct Vector{
    i:f32,
    j:f32,
    k:f32,
    magnitude:f32,
}

impl Vector {
    pub fn new(i:f32, j:f32, k:f32) -> Self {
        Self{
            i:i,
            j:j,
            k:k,
            magnitude:(i * i + j*j + k*k).sqrt(),
        }
    }

    pub fn from(first:Point, second:Point) -> Self {
        Self{
            i:(first.get_x() - second.get_x()),
            j:(first.get_y() - second.get_y()),
            k:(first.get_z() - second.get_z()),
        }
    }

    pub fn cross(&self, other:Vector) -> Vector {
        let new_i:f32 = self.get_j() * other.get_k() - self.get_k() * other.get_j();
        let new_j:f32 = self.get_k() * other.get_i() - self.get_i() * other.get_k();
        let new_k:f32 = self.get_i() * other.get_j() - self.get_j() * other.get_i();
        Vector::new(new_i, new_j, new_k)
    }

    pub fn dot(&self, other:Vector) -> f32 {
        self.get_i() * other.get_i() + self.get_j() * other.get_j() + self.get_k() * other.get_k()
    }

    pub fn angle(&self, other:Vector) -> f32 {
        let mut a:f32 = (self.dot(other) / (self.get_magnitude() * other.get_magnitude)).acos()
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

    pub fn get_magnitude(&self) -> f32 {
        self.magnitude
    }
}
