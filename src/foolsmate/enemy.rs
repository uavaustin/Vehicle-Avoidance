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

    // Mutators
    pub fn update_location(&mut self, new_location: Point) {
        self.prev_location = self.location;
        self.location = new_location;
        self.update_speed();
        self.update_dir();
    }

    fn update_speed(&mut self) {
        self.speed = ((self.location.get_x() - self.prev_location.get_x()).powi(2) +
            (self.location.get_y() - self.prev_location.get_y()).powi(2)).sqrt()
    }

    pub fn set_speed(&mut self, new_speed:f32) {
        self.speed = new_speed;
    }

    fn update_dir(&mut self) {
        const NORTH:f32 = 0f32;
        const SOUTH:f32 = 180f32;
        const EAST:f32 = 90f32;
        const WEST:f32 = 270f32;

        let dx:f32 = self.location.get_x() - self.prev_location.get_x();
        let dy:f32 = self.location.get_y() - self.prev_location.get_y();

        //print!("\ndx: {}, dy: {}\n", dx, dy);
        if dx == 0f32 {
            if dy < 0f32 {
                self.dir = SOUTH;
            } else if dy > 0f32 {
                self.dir = NORTH;
            }
        } else if dy == 0f32 {
            if dx < 0f32 {
                self.dir = WEST;
            } else if dx > 0f32 {
                self.dir = EAST;
            }
        } else {
            //print!("\nNOT CARDINAL\n");
            let theta = (dy/dx).abs().atan().to_degrees();
            if dy > 0f32 && dx > 0f32 {
                self.dir = EAST-theta;
            } else if dy > 0f32 && dx < 0f32 {
                self.dir = WEST + theta;
            } else if dy < 0f32 && dx < 0f32 {
                self.dir = WEST - theta;
            } else {
                self.dir = EAST+theta;
            }
        }
    }

    pub fn set_dir(&mut self, new_dir: f32) {
        self.dir = new_dir
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create() {
        let s = 3.0f32;
        let dir = 180f32;
        let loc = Point::new(&1f32, &1f32, &1f32);
        let en = Enemy::new(s, dir, loc);

        assert_eq!(en.get_location(), loc);
        assert_eq!(en.get_dir(), dir);
        assert_eq!(en.get_speed(), s);
    }

    #[test]
    fn test_update_location() {
        let s = 3.0f32;
        let dir = 180f32;
        let loc = Point::new(&1f32, &1f32, &1f32);
        let new_loc = Point::new(&2f32,&1f32,&1f32);
        let mut en = Enemy::new(s, dir, loc);

        en.update_location(new_loc);

        assert_eq!(en.get_prev_location(), loc);
        assert_eq!(en.get_location(), new_loc);
    }

    #[test]
    fn test_update_cardinal_dir() {
        let s = 3.0f32;
        let dir = 180f32;
        let loc = Point::new(&1f32, &1f32, &1f32);
        let mut en = Enemy::new(s, dir, loc);
        let mut new_loc = Point::new(&2f32,&1f32,&1f32);

        en.update_location(new_loc);
        assert_eq!(en.get_dir(), 90f32);

        new_loc = Point::new(&2f32,&2f32,&1f32);
        en.update_location(new_loc);
        assert_eq!(en.get_dir(), 0f32);

        new_loc = Point::new(&2f32,&1f32,&1f32);
        en.update_location(new_loc);
        assert_eq!(en.get_dir(), 180f32);

        new_loc = Point::new(&1f32,&1f32,&1f32);
        en.update_location(new_loc);
        assert_eq!(en.get_dir(), 270f32);
    }

    #[test]
    fn test_update_diagonal() {
        let s = 3.0f32;
        let dir = 180f32;
        let loc = Point::new(&1f32, &1f32, &1f32);
        let mut en = Enemy::new(s, dir, loc);
        let mut dire:f32  = 1f32 + ((3f32/4f32) as f32).sqrt();

        let mut new_loc = Point::new(&dire,&1.5f32,&1f32);

        en.update_location(new_loc);
        assert_eq!(en.get_dir() - 60f32 <= 0.01, true);

        dire = 1f32 + ((3f32/4f32) as f32).sqrt();
        new_loc = Point::new(&1f32,&1f32,&1f32);
        en.update_location(new_loc);
        new_loc = Point::new(&1.5f32,&dire,&1f32);
        en.update_location(new_loc);
        assert_eq!(en.get_dir() - 30f32 <= 0.01 , true);

        dire = 1f32 - ((3f32/4f32) as f32).sqrt();
        new_loc = Point::new(&1f32,&1f32,&1f32);
        en.update_location(new_loc);
        new_loc = Point::new(&dire,&1.5f32,&1f32);
        en.update_location(new_loc);
        assert_eq!(en.get_dir() - 300f32 <= 0.01, true);


        dire = 1f32 - ((3f32/4f32) as f32).sqrt();
        new_loc = Point::new(&1f32,&1f32,&1f32);
        en.update_location(new_loc);
        new_loc = Point::new(&1.5f32,&dire,&1f32);
        en.update_location(new_loc);
        assert_eq!(en.get_dir() - 150f32 <= 0.01, true);
    }
}
