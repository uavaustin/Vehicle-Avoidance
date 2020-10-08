#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point{
    //Assuming x,y,z are always positive
    x: f32,
    y: f32,
    z: f32,
}

impl Point{
    pub fn new(x: &f32, y: &f32, z: &f32) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
        }
    }

    pub fn blank() -> Self{
        Self {
            x: -1f32,
            y: -1f32,
            z: -1f32,
        }
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

        let p = Point::new(&x, &y, &z);

        assert_eq!(p.get_x() == x && p.get_y() == y && p.get_z() == z, true)
    }

    #[test]
    fn create_empty() {
        let p = Point::blank();
        assert_eq!(p.get_x() == -1f32 && p.get_y() == -1f32 && p.get_z() == -1f32, true)
    }
}
