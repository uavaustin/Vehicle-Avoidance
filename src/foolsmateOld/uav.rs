use super::point::*;

//TODO: Add pitch
#[derive(Clone, Debug, PartialEq)]
pub struct UAV {
    speed: f32,
    heading: f32,
    location: Point,
    path: Vec<Point>,
}

//TODO: Add pitch
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
        self.speed
    }

    pub fn get_heading(&self) -> f32 {
        self.heading
    }

    pub fn get_pitch(&self) -> f32 {
        0f32
    }

    pub fn get_location(&self) -> Point {
        self.location
    }

    pub fn get_path(&self) -> Vec<Point> {
        self.path.clone()
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

        let dx:f32 = self.to_cardinal_degrees().to_radians().cos() * self.speed * TIME;
        let dy:f32 = self.to_cardinal_degrees().to_radians().sin() * self.speed * TIME;

        self.location = Point::new(&(self.location.get_x() + dx), &(self.location.get_y() + dy), &self.location.get_y());
        self.update_path();
    }

    fn update_path(&mut self) {
        //Distance from the point needed to remove it from the list
        const RADIUS:f32 = 0.5;
        if self.path.len() != 0 {
            let current_loc:Point = self.location;
            let next_point:Point = self.path[0];

            if (current_loc.get_x() - next_point.get_x()).abs() < RADIUS &&
                (current_loc.get_y() - next_point.get_y()).abs() < RADIUS
            {
                self.path.remove(0);
            }
        }


    }

    fn to_cardinal_degrees(&self) -> f32 {
        450f32-self.heading
    }
}

//TODO: Add pitch
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uav_create() {
        let speed:f32 = 1f32;
        let heading:f32 = 0f32;
        let location:Point = Point::new(&0f32, &0f32, &10f32);
        let mut path:Vec<Point> = Vec::new();

        for x in 0..10 {
            path.push(Point::new(&(x as f32), &(x as f32), &10f32));
        }

        let uav = UAV::new(speed, heading, location, path);

        assert_eq!(uav.get_location(), location);
    }

    #[test]
    fn test_uav_fly() {
        let speed:f32 = 1f32;
        let heading:f32 = 0f32;
        let location:Point = Point::new(&0f32, &0f32, &10f32);
        let mut path:Vec<Point> = Vec::new();

        for x in 0..10 {
            path.push(Point::new(&(x as f32), &(x as f32), &10f32));
        }

        let mut uav = UAV::new(speed, heading, location, path);
        uav.update_location();
        assert_eq!((uav.location.get_x() - 0f32).abs() < 0.1, true);
        assert_eq!((uav.location.get_y() - 0.5f32).abs() < 0.1, true);
    }

    #[test]
    fn test_path_remove() {
        let speed:f32 = 1f32;
        let heading:f32 = 0f32;
        let location:Point = Point::new(&0f32, &0f32, &10f32);
        let mut path:Vec<Point> = Vec::new();
        path.push(Point::new(&0f32, &0.5f32, &10f32));
        let mut uav = UAV::new(speed, heading, location, path);
        assert_eq!(uav.path.len(), 1);
        uav.update_location();
        assert_eq!(uav.path.len(), 0);

    }
}
