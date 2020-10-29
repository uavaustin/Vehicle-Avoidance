use obj::craft::Craft;
use obj::location::Location;
use space::point::Point;
use std::collections::LinkedList;
use space::vector::Vector;
use space::quaternion::Quaternion;

pub struct FoolsMate {
    path: LinkedList<Point>,
    enemy_point: Point
    uav_point: Point,
    enemy_heading: Vector,
    uav_heading: Vector,
    rotation: Quaternion,
}

/*
NOTE: NEED TO ENSURE EACH CRAFT'S SPEED IS SCALED PROPERLY
all location data is scaled according to equatorial calculations
But telemetry data is not. 

Either need to change scale factor or scale the telemetry data
*/
impl FoolsMate {
    pub fn new(uav: Craft, enemy: Craft, path: LinkedList<Node>) -> Self {
        const OUTER_TIME_IN_SECS:f32 = 30f32;
        const INNER_TIME_IN_SECS:f32 = 10f32;
        const THETA:f32 = std::f32::consts::PI / 4f32;

        let ref_point = Point::define_ref(enemy.get_location());
        //Set enemy to be (0,0,0)
        let enemy_point = Point::from_location(enemy.get_location(), ref_point)
        let uav_point = Point::from_location(uav.get_location(), ref_point);
        //Rotation axis perpendicular to X-axis and direction vector of enemy
        let x_axis:Vector = Vector::new(1f32,0f32,0f32);
        let rotation_axis:Vector = x_axis.cross(enemy.get_heading());
        let angle:f32 = enemy.get_heading().angle(x_axis) / 2f32;
        let rotation:Quaternion = Quaternion::rotation(angle, rotation_axis);
        let mut point_path:LinkedList<Point> = LinkedList::new();

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
            rotation : rotation,
        }
    }

    fn withinOuterSphere(&self) -> bool {
        let ENEMY_SPEED:f32 = self.uav.get_heading().magnitude();
        let MAX_DIST:f32 = ENEMY_SPEED * OUTER_TIME_IN_SECS;
        self.uav_point.dist(self.enemy_point) <= MAX_DIST;
    }

    fn withinInnerSphere(&self) -> bool {
        let ENEMY_SPEED:f32 = self.uav.get_heading().magnitude();
        let MAX_DIST:f32 = ENEMY_SPEED * INNER_TIME_IN_SECS;
        self.uav_point.dist(self.enemy_point) <= MAX_DIST;
    }
    
    //Postcondition: XYZ space is rotated such that the enemy heading is directly oriented along the x-axis
    fn rotate_space(&mut self) {
        self.uav_point = Point::from_vector(self.rotation.rotate_vector(Vector::from_point(self.uav_point)));
        //Does rotating the heading vector return the correct vector? 
        self.uav_heading = self.rotation.rotate_vector(self.uav_heading);
        self.enemy_heading = self.rotation.rotate_vector(self.enemy_heading);

        for point in self.path.iter_mut() {
            point = self.rotation.rotate_vector(Vector::from_point(point));
        }
    }

    //Check if the fastest time it could reach (within the sector) is < time to exit the sector?
    fn within_sector(&self) -> bool {
        
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


    */
    
    #[test]
    fn test_rotation() {
        
    }
}