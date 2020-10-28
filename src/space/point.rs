use obj::location::Location;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    //Assuming x,y,z are always positive
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

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
}
