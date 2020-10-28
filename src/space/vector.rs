use super::point::*;
use std::f32;
use std::ops::Mul;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vector {
    i: f32,
    j: f32,
    k: f32,
    magnitude: f32,
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        let new_i = self.i * rhs;
        let new_j = self.j * rhs;
        let new_k = self.k * rhs;

        Vector::new(new_i, new_j, new_k)
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        let new_i: f32 = self.get_j() * rhs.get_k() - self.get_k() * rhs.get_j();
        let new_j: f32 = self.get_k() * rhs.get_i() - self.get_i() * rhs.get_k();
        let new_k: f32 = self.get_i() * rhs.get_j() - self.get_j() * rhs.get_i();
        Vector::new(new_i, new_j, new_k)
    }
}

impl Vector {
    pub fn new(i: f32, j: f32, k: f32) -> Self {
        Self {
            i: i,
            j: j,
            k: k,
            magnitude: (i * i + j * j + k * k).sqrt(),
        }
    }

    pub fn from(first: Point, second: Point) -> Self {
        Self {
            i: -1f32 * (first.get_x() - second.get_x()),
            j: -1f32 * (first.get_y() - second.get_y()),
            k: -1f32 * (first.get_z() - second.get_z()),
            magnitude: ((first.get_x() - second.get_x()) * (first.get_x() - second.get_x())
                + (first.get_y() - second.get_y()) * (first.get_y() - second.get_y())
                + (first.get_z() - second.get_z()) * (first.get_z() - second.get_z()))
            .sqrt(),
        }
    }

    pub fn dot(&self, other: Vector) -> f32 {
        self.get_i() * other.get_i() + self.get_j() * other.get_j() + self.get_k() * other.get_k()
    }

    pub fn angle(&self, other: Vector) -> f32 {
        let mut a: f32 = (self.dot(other) / (self.get_magnitude() * other.get_magnitude())).acos();
        if a > f32::consts::PI / 2f32 {
            a = a - f32::consts::PI / 2f32;
        }
        a.to_degrees()
    }

    pub fn to_dir(&self) -> Vector {
        let scalar: f32 = 1f32 / self.get_magnitude();
        let temp: Vector = Vector::new(self.get_i(), self.get_j(), self.get_k());
        temp * scalar
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_new() {
        let v = Vector::new(1f32, 1f32, 1f32);
        assert_eq!(
            v.get_i() == 1f32 && v.get_j() == 1f32 && v.get_k() == 1f32,
            true
        )
    }
    #[test]
    fn create_from() {
        let p1 = Point::new(1f32, 1f32, 1f32);
        let p2 = Point::new(3f32, 2f32, 1f32);
        let v = Vector::from(p1, p2);
        assert_eq!(
            v.get_i() == 2f32 && v.get_j() == 1f32 && v.get_k() == 0f32,
            true
        );
    }

    #[test]
    fn test_products() {
        let v1: Vector = Vector::new(1f32, 1f32, 1f32);
        let v2: Vector = Vector::new(2f32, 3f32, 4f32);

        assert_eq!(v1.dot(v2), 9f32);

        let cross: Vector = v1 * v2;
        assert_eq!(
            cross.get_i() == 1f32 && cross.get_j() == -2f32 && cross.get_k() == 1f32,
            true
        );

        let scalar: f32 = 2f32;
        let scaled: Vector = v2 * scalar;
        assert_eq!(
            scaled.get_i() == 4f32 && scaled.get_j() == 6f32 && scaled.get_k() == 8f32,
            true
        );
    }
}
