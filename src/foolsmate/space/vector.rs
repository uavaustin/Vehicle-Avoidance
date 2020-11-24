use super::point::*;
use std::f32;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Debug, Copy)]
pub struct Vector {
    i: f32,
    j: f32,
    k: f32,
    magnitude: f32,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector:({}, {}, {})\n", self.i, self.j, self.k)
    }
}

impl std::cmp::PartialEq for Vector {
    fn eq(&self, rhs: &Vector) -> bool {
        const ACCEPTABLE_MARGIN: f32 = 0.01f32;
        let i_eq: bool = (rhs.get_i() - self.get_i()) == 0f32
            || ((rhs.get_i() - self.get_i()) / self.get_i()) < ACCEPTABLE_MARGIN;

        let j_eq: bool = (rhs.get_j() - self.get_j()) == 0f32
            || ((rhs.get_j() - self.get_j()) / self.get_j()) < ACCEPTABLE_MARGIN;
        let k_eq: bool = (rhs.get_k() - self.get_k()) == 0f32
            || ((rhs.get_k() - self.get_k()) / self.get_k()) < ACCEPTABLE_MARGIN;

        i_eq && j_eq && k_eq
    }
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

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let new_i: f32 = self.get_i() - rhs.get_i();
        let new_j: f32 = self.get_j() - rhs.get_j();
        let new_k: f32 = self.get_k() - rhs.get_k();
        Vector::new(new_i, new_j, new_k)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        let new_i: f32 = self.get_i() + rhs.get_i();
        let new_j: f32 = self.get_j() + rhs.get_j();
        let new_k: f32 = self.get_k() + rhs.get_k();
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

    pub fn from_point(point: Point) -> Self {
        Self {
            i: point.get_x(),
            j: point.get_y(),
            k: point.get_z(),
            magnitude: (point.get_x() * point.get_x()
                + point.get_y() * point.get_y()
                + point.get_z() * point.get_z())
            .sqrt(),
        }
    }

    pub fn dot(&self, other: Vector) -> f32 {
        self.get_i() * other.get_i() + self.get_j() * other.get_j() + self.get_k() * other.get_k()
    }

    pub fn cross(first: Vector, second: Vector) -> Vector {
        let i_new: f32 = (first.get_j() * second.get_k()) - (first.get_k() * second.get_j());
        let j_new: f32 = (first.get_k() * second.get_i()) - (first.get_i() * second.get_k());
        let k_new: f32 = (first.get_i() * second.get_j()) - (first.get_j() * second.get_i());
        let norm: Vector = Vector::new(i_new, j_new, k_new);
        norm
    }

    pub fn intersect(point1: Vector, heading1: Vector, point2: Vector, heading2: Vector) -> Vector {
        // find the direction projection
        let u: f32 = heading1.dot(heading2);
        if u == 1f32 {
            // the lines are parallel
            return Vector::new(0f32, 0f32, -1f32);
        }
        //Find the separation projections
        let t1: f32 = (point2.sub(point1)).dot(heading1);
        let t2: f32 = (point2.sub(point1)).dot(heading2);
        //Find distance along line1
        let d1: f32 = (t1 - u * t2) / (1f32 - u * u);
        let d2: f32 = (t2 - u * t1) / (u * u - 1f32);
        //Find the point on line1
        let p1: Vector = point1.add(heading1.scale(d1));
        let p2: Vector = point2.add(heading2.scale(d2));

        let norm: Vector = Vector::cross(p1, p2);
        norm
    }

    pub fn add(&self, other: Vector) -> Vector {
        let added: Vector = Vector::new(
            self.get_i() + other.get_i(),
            self.get_j() + other.get_j(),
            self.get_k() + other.get_k(),
        );
        added
    }

    pub fn sub(&self, other: Vector) -> Vector {
        let sub: Vector = Vector::new(
            self.get_i() - other.get_i(),
            self.get_j() - other.get_j(),
            self.get_k() - other.get_k(),
        );
        sub
    }

    pub fn scale(&self, other: f32) -> Vector {
        let scaled: Vector = Vector::new(
            other * self.get_i(),
            other * self.get_j(),
            other * self.get_k(),
        );
        scaled
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

        let cross : Vector = Vector::cross(v1, v2);
        assert_eq!(
            cross.get_i() == 1f32 && cross.get_j() == -2f32 && cross.get_k() == 1f32,
            true
        );

        let scalar: f32 = 2f32;
        let scaled: Vector = v2.scale( scalar);
        assert_eq!(
            scaled.get_i() == 4f32 && scaled.get_j() == 6f32 && scaled.get_k() == 8f32,
            true
        );

        let sum: Vector = v1.add(v2);
        assert_eq!(
            sum.get_i() == 3f32 && sum.get_j() == 4f32 && sum.get_k() == 5f32,
            true
        );

        let dif: Vector = v1.sub(v2);
        assert_eq!(
            dif.get_i() == -1f32 && dif.get_j() == -2f32 && dif.get_k() == -3f32,
            true
        );


    }
}
