use super::point::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enemy{
    speed: f32,
    dir: f32,
    location: Point,
    prev_location: Point,
}

impl Enemy{
    pub fn new(speed:f32, dir:f32, location:Point) -> Self {
        Self {
            speed: speed,
            dir: dir,
            location: location,
            prev_location: Point::blank()
        }
    }

    // Mutators
    pub fn update_location(&mut self, new_location: Point) {
        self.prev_location = self.location;
        self.location = new_location;
    }

    //Accessors
    pub fn get_location(&self) -> Point {
        self.location
    }

    pub fn get_prev_location(&self) -> Point {
        self.prev_location
    }

    pub fn get_dir(&self) -> f32 {
        self.dir
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create() {
        let s = 3.0f32;
        let dir = 180f32;
        let loc = Point::new(1f32, 1f32, 1f32);
        let en = Enemy::new(s, dir, loc);

        assert_eq!(en.get_location(), loc);
        assert_eq!(en.get_dir(), dir);
        assert_eq!(en.get_speed(), s);
    }

    #[test]
    fn test_update() {
        let s = 3.0f32;
        let dir = 180f32;
        let loc = Point::new(1f32, 1f32, 1f32);
        let new_loc = Point::new(2f32,1f32,1f32);
        let mut en = Enemy::new(s, dir, loc);

        en.update_location(new_loc);

        assert_eq!(en.get_prev_location(), loc);
        assert_eq!(en.get_location(), new_loc);
    }
}
