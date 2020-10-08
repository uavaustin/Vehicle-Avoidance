use super::point::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UAV {
    speed: f32,
    heading: f32,
    location: Point,
    path: Vec<Point>,
}

impl UAV {
    pub fn new(speed: f32, heading: f32, location: Point, path: Vec<Point>) -> Self {
        Self {
            speed: speed,
            heading: heading,
            location: location,
            path: path,
        }
    }

    //Accessors
    pub fn get_speed(&self) -> f32 {
        self.speed;
    }

    pub fn get_heading(&self) -> f32 {
        self.heading;
    }

    pub fn get_location(&self) -> Point {
        self.location;
    }

    pub fn get_path(&self) -> Vec<Point> {
        self.path;
    }

    //Mutators
    pub fn set_speed(&mut self, new_speed: f32) {
        self.speed = new_speed;
    }

    pub fn set_heading(&mut self, new_heading: f32) {
        self.heading = new_heading;
    }

    pub fn set_location(&mut self, new_loc:Point) {
        self.location = new_loc;
    }

    pub fn update_location(&mut self) {
        //Assuming half-second updates -> Can be modified to match input frequency
        const TIME:f32 = 0.5f32;

        let dx:f32 = self.to_degrees().cos() * self.speed * TIME;
        let dy:f32 = self.to_degrees().sin() * self.speed * TIME;

        self.location = Point::new(self.location.get_x() + dx, self.location.get_y() + dy);
    }

    fn to_degrees(&self) -> f32 {
        450-self.heading
    }
}
