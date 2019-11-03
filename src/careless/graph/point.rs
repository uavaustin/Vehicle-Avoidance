//mod obj;

const RADIUS: f64 = 6_371_000.0;

use obj::location::Location;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl From<(&Location, &Location)> for Point {
    fn from((location, origin): (&Location, &Location)) -> Self {
        Self::new(
            2f32
                * RADIUS as f32
                * (location.lat().cos() * ((location.lon() - origin.lon()) / 2f32).sin()).asin(),
            RADIUS as f32 * (location.lat() - origin.lat()),
            location.alt(),
        )
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn dist(&self, p2: &Point) -> f32 {
        ((self.x - p2.x).powi(2) + (self.y - p2.y).powi(2) + (self.z - p2.z).powi(2)).sqrt()
    }

    pub fn x(&self) -> f32 {
        self.x.into()
    }

    pub fn y(&self) -> f32 {
        self.y.into()
    }

    pub fn z(&self) -> f32 {
        self.z.into()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_point_new() {
        let (x, y, z) = (0f32, 4f32, 7f32);
        let p = Point::new(x, y, z);
        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
        assert_eq!(p.z, z);
    }

    #[test]
    fn test_point_from_location() {
        let loc = Location::new(0.00001f32, 0.00001f32, 1f32);
        let o = Location::new(0f32, 0f32, 0f32);
        let p = Point::from((&loc, &o));
        assert_eq!(p.x, 63.71f32);
        assert_eq!(p.y, 63.71f32);
        assert_eq!(p.z, 1f32);
    }
}
