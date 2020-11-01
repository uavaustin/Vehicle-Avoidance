use foolsmate::node::Node;
use foolsmate::space::point::Point;
use foolsmate::space::quaternion::Quaternion;
use foolsmate::space::vector::Vector;
use obj::craft::Craft;
use obj::location::Location;
use std::collections::LinkedList;

pub struct FoolsMate {
    path: LinkedList<Point>,
    enemy_point: Point,
    uav_point: Point,
    enemy_heading: Vector,
    uav_heading: Vector,
    rotation: Quaternion,
    ref_point: Point,
}

/*
NOTE: NEED TO ENSURE EACH CRAFT'S SPEED IS SCALED PROPERLY
all location data is scaled according to equatorial calculations
But telemetry data is not.

Either need to change scale factor or scale the telemetry data
*/
impl FoolsMate {
    const THETA: f32 = std::f32::consts::PI / 4f32;

    pub fn new(uav: Craft, enemy: Craft, path: LinkedList<Node>) -> Self {
        let ref_point = Point::define_ref(enemy.get_location());
        //Set enemy to be (0,0,0)
        let enemy_point = Point::from_location(enemy.get_location(), ref_point);
        let uav_point = Point::from_location(uav.get_location(), ref_point);
        //Axis THETA/2 radians offset from X-axis along the xy plane
        let mid_axis: Vector = Vector::new(
            (FoolsMate::THETA / 2f32).cos(),
            (FoolsMate::THETA / 2f32).sin(),
            0f32,
        );
        //Rotates enemy heading such that it is THETA/2 degrees away from the x-axis
        let rotation_axis: Vector = mid_axis * (enemy.get_heading());
        let angle: f32 = enemy.get_heading().angle(mid_axis) / 2f32;
        let rotation: Quaternion = Quaternion::rotation(angle, rotation_axis);
        let mut point_path: LinkedList<Point> = LinkedList::new();

        for node in path.iter() {
            point_path.push_back(Point::from_location(node.get_location(), ref_point));
        }
        Self {
            path: point_path,
            ref_point: ref_point,
            enemy_point: enemy_point,
            uav_point: uav_point,
            enemy_heading: enemy.get_heading(),
            uav_heading: uav.get_heading(),
            rotation: rotation,
        }
    }

    pub fn define(
        ref_point: Point,
        enemy_point: Point,
        uav_point: Point,
        enemy_heading: Vector,
        uav_heading: Vector,
        path: LinkedList<Point>,
        rotation: Quaternion,
    ) -> Self {
        Self {
            path: path,
            ref_point: ref_point,
            enemy_point: enemy_point,
            uav_point: uav_point,
            enemy_heading: enemy_heading,
            uav_heading: uav_heading,
            rotation: rotation,
        }
    }

    //Postcondition: XYZ space is rotated such that the enemy heading is directly oriented along the x-axis
    fn rotate_space(&mut self) {
        self.uav_point = Point::from_vector(
            self.rotation
                .rotate_vector(Vector::from_point(self.uav_point)),
        );
        //Does rotating the heading vector return the correct vector?
        self.uav_heading = self.rotation.rotate_vector(self.uav_heading);
        self.enemy_heading = self.rotation.rotate_vector(self.enemy_heading);

        for point in self.path.iter_mut() {
            *point = Point::from_vector(self.rotation.rotate_vector(Vector::from_point(*point)));
        }
    }

    //Checks if UAV is within triangle defined by x-axis and the outer arm of the path
    //Return true iff you are within the sector since we have already checked the radius
    fn uav_within_sector(&self) -> bool {
        self.uav_point.get_y() <= (FoolsMate::THETA / 2f32).tan() * self.uav_point.get_x()
            && self.uav_point.get_z() <= (FoolsMate::THETA / 2f32).tan() * self.uav_point.get_x()
    }

    //Checks if UAV needs to change course while it is within a sector of the cylinder
    fn change_course(&self) -> bool {
        let ENEMY_SPEED: f32 = self.enemy_heading.get_magnitude();
        let UAV_SPEED: f32 = self.uav_heading.get_magnitude();

        // * Add code to get the exit point from next WayPoint D(x2,y2,z2)
        // * Initialize uav_point_exit of type Point

        //ADDED AS PLACEHOLDER
        let uav_point_exit = Point::new(0f32, 0f32, 0f32);
        let dist_vec: Vector = Vector::from(self.uav_point, uav_point_exit);
        let dist: f32 = dist_vec.get_magnitude();
        let dist_unit_vec: Vector = dist_vec.to_dir();

        let vel_vec: Vector = dist_unit_vec * UAV_SPEED;
        let vel: f32 = vel_vec.get_magnitude();

        // Assuming constant velocity (neglecting drag for now)
        let uav_path_time: f32 = dist / vel;
        false
    }

    //Possible Optimisation: Find closest point on cone and take normal

    //A waypoint above/below the enemy plane and (slightly) within the sphere
    //To account for the enemy moving between checks
    fn evade(&mut self, vertical: bool) {
        const HORIZONTAL_INSET: f32 = 0.5f32;
        //2 meters above or below
        let vertical: f32 = 2f32 * ((vertical as i8) * 2) as f32 - 1f32;
        let horizontal: f32 = vertical / FoolsMate::THETA.tan() + HORIZONTAL_INSET;
        let new_waypoint: Point = Point::new(horizontal, horizontal, vertical);
        let first_point: Option<Point> = self.path.pop_front();
        self.path.push_front(new_waypoint);
        match first_point {
            Some(p) => self.path.push_front(p),
            None => (),
        }
    }
}

//Check if closest point on current path of enemy plane is within the sector?
//pub func withinSector(self:Craft, enemy:Craft) -> bool

//pub func evadeClose(self:Craft, enemy:Craft) -> Vector<??>

#[cfg(test)]
mod tests {

    use super::*;
    fn create_fm() -> FoolsMate {
        let mut path: LinkedList<Point> = LinkedList::new();
        for x in 0..10 {
            path.push_back(Point::new(10f32, x as f32, 0f32));
        }
        let uav_point: Point = Point::new(10f32, 0f32, 0f32);
        let enemy_point: Point = Point::new(0f32, 0f32, 0f32);
        let ref_point: Point = Point::new(0f32, 0f32, 0f32);
        let enemy_heading: Vector = Vector::new(1f32, 0f32, 0f32);
        let uav_heading: Vector = Vector::new(0f32, 1f32, 1f32);
        let axis: Vector = Vector::new(0f32, 0f32, 1f32);
        let angle: f32 = std::f32::consts::PI / 4f32;
        let rotation: Quaternion = Quaternion::rotation(angle, axis);
        FoolsMate::define(
            ref_point,
            enemy_point,
            uav_point,
            enemy_heading,
            uav_heading,
            path,
            rotation,
        )
    }
    #[test]
    fn test_rotation() {
        let mut fm: FoolsMate = create_fm();
        fm.rotate_space();
        assert_eq!(fm.uav_point, Point::new(0f32, 10f32, 0f32));
        let mut path: LinkedList<Point> = fm.path;
        for i in 0..path.len() {
            let left = path.pop_front();
            let right: Point = Point::new(-1f32 * i as f32, 10f32, 0f32);
            assert_eq!(left, Some(right));
        }
    }
}
