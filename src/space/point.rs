use obj::location::Location;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    //Assuming x,y,z are always positive
    x: f32,
    y: f32,
    z: f32,
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        self.x == rhs.get_x() && self.y == rhs.get_y() && self.z == rhs.get_z()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point:({}, {}, {}) \n", self.x, self.y, self.z)
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    //NOTE -> THESE TRANSFORMATIONS ARE THE VALUES AT THE EQUATOR AND MAY NOT EXACTLY CORRESPOND TO SCALE IRL
    pub fn from_location(location: Location, ref_point: Point) -> Self {
        /*
        1 unit = 1 m
        1º = 111.32 km = 111 320 m at the equator
        initial x-axis: latitude º
        initial y-axis: longitude º
        initial z-axis: altitude m
        */
        const SCALE_FACTOR: f32 = 111_320f32;
        let x: f32 = location.lat() * SCALE_FACTOR - ref_point.get_x();
        let y: f32 = location.lon() * SCALE_FACTOR - ref_point.get_y();
        let z: f32 = location.alt() - ref_point.get_z();

        Self { x: x, y: y, z: z }
    }

    pub fn define_ref(location: Location) -> Self {
        const SCALE_FACTOR: f32 = 111_320f32;
        let x: f32 = location.lat() * SCALE_FACTOR;
        let y: f32 = location.lon() * SCALE_FACTOR;
        let z: f32 = location.alt();
        Self { x: x, y: y, z: z }
    }

    pub fn dist(&self, second: Point) -> f32 {
        let x_diff: f32 = (second.get_x() - self.get_x()).powi(2);
        let y_diff: f32 = (second.get_y() - self.get_y()).powi(2);
        let z_diff: f32 = (second.get_z() - self.get_z()).powi(2);
        (x_diff + y_diff + z_diff).sqrt()
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_populated() {
        let x = 1.28f32;
        let y = 9.97f32;
        let z = 1.17f32;

        let p = Point::new(x, y, z);

        assert_eq!(p.get_x() == x && p.get_y() == y && p.get_z() == z, true)
    }

    #[test]
    fn create_ref() {
        let x: f32 = 1f32;
        let y: f32 = 3f32;
        let z: f32 = 5f32;
        let p: Point = Point::define_ref(Location::new(x, y, z));
        assert_eq!(p, Point::new(111_320f32, 333_960f32, 5f32));
    }

    #[test]
    fn compare_ref() {
        let lat: f32 = 1f32;
        let lon: f32 = 3f32;
        let alt: f32 = 5f32;
        let p: Point = Point::define_ref(Location::new(lat, lon, alt));
        let x: f32 = 1f32;
        let y: f32 = 2f32;
        let z: f32 = 3f32;
        let p2: Point = Point::from_location(Location::new(x, y, z), p);
        assert_eq!(p2, Point::new(0f32, -111_320f32, -2f32));
    }
}
