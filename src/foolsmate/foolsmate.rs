use foolsmate::node::Node;
use obj::craft::Craft;
use obj::location::Location;
use space::point::Point;
use space::quaternion::Quaternion;
use space::vector::Vector;
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
    const OUTER_TIME_IN_SECS: f32 = 30f32;
    const INNER_TIME_IN_SECS: f32 = 10f32;

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

    fn within_inner_sphere(&self) -> bool {
        let enemy_speed: f32 = self.enemy_heading.get_magnitude();
        let max_dist: f32 = enemy_speed * FoolsMate::OUTER_TIME_IN_SECS;
        self.uav_point.dist(self.enemy_point) <= max_dist
    }

    fn within_outer_sphere(&self) -> bool {
        let enemy_speed: f32 = self.enemy_heading.get_magnitude();
        let max_dist: f32 = enemy_speed * FoolsMate::INNER_TIME_IN_SECS;
        self.uav_point.dist(self.enemy_point) <= max_dist
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
    fn uav_within_sector(&self) -> bool {
        self.uav_point.get_y() <= (FoolsMate::THETA / 2f32).tan() * self.uav_point.get_x()
            && self.uav_point.get_z() <= (FoolsMate::THETA / 2f32).tan() * self.uav_point.get_x()
    }
}

//impl FoolsMate

//Check if closest point on current path of enemy plane is within the sector?
//pub func withinSector(self:Craft, enemy:Craft) -> bool

//pub func evade(self:Craft, enemy:Craft) -> Vector<??>

//pub func evadeClose(self:Craft, enemy:Craft) -> Vector<??>

#[cfg(test)]
mod tests {

    use super::*;

    /* What needs to be tested:
    Is the rotation angle for the quaternion backwards (turns in opposite direction)

    Do the rotations work????
    */
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
