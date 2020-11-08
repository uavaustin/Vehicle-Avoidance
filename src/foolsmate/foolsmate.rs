use foolsmate::node::Node;
use foolsmate::space::point::Point;
use foolsmate::space::quaternion::Quaternion;
use foolsmate::space::vector::Vector;
use foolsmate::sphere::Sphere;
use obj::craft::Craft;
use obj::location::Location;
use std::collections::LinkedList;
use std::ops::Mul;

pub struct FoolsMate {
    path: LinkedList<Point>,
    enemy_point: Point,
    uav_point: Point,
    enemy_heading: Vector,
    uav_heading: Vector,
    rotation: Quaternion,
    ref_point: Point,
    sphere: Sphere,
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
        let sphere: Sphere = Sphere::new(enemy.get_heading());
        Self {
            path: point_path,
            ref_point: ref_point,
            enemy_point: enemy_point,
            uav_point: uav_point,
            enemy_heading: enemy.get_heading(),
            uav_heading: uav.get_heading(),
            rotation: rotation,
            sphere: sphere,
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
        sphere: Sphere,
    ) -> Self {
        Self {
            path: path,
            ref_point: ref_point,
            enemy_point: enemy_point,
            uav_point: uav_point,
            enemy_heading: enemy_heading,
            uav_heading: uav_heading,
            rotation: rotation,
            sphere: sphere,
        }
    }

    fn foolsmate(&mut self) -> &LinkedList<Point> {
        if self
            .sphere
            .within_outer_sphere(self.uav_point, self.enemy_point)
        {
            self.rotate_space();
            if self.uav_within_sector(FoolsMate::THETA) && self.change_course() {
                self.evade();
            } else if self
                .sphere
                .within_inner_sphere(self.uav_point, self.enemy_point)
            {
                self.evade_close();
            }
        }
        &self.path
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

    fn revert_space(&mut self) {
        self.rotation = self.rotation.get_conjugate();
        self.rotate_space();
    }

    //Checks if UAV is within triangle defined by x-axis and the outer arm of the path
    //Return true iff you are within the sector since we have already checked the radius
    fn uav_within_sector(&self, angle: f32) -> bool {
        self.uav_point.get_x() >= 0f32
            && self.uav_point.get_y() >= 0f32
            && self.uav_point.get_y() <= (angle / 2f32).tan() * self.uav_point.get_x()
            && self.uav_point.get_z() <= (angle / 2f32).tan() * self.uav_point.get_x()
    }

    //Checks if UAV needs to change course while it is within a sector of the cylinder
    fn change_course(&self) -> bool {
        //Should we be using velocity instead of heading here?
        let ENEMY_SPEED: f32 = self.enemy_heading.get_magnitude();
        let UAV_SPEED: f32 = self.uav_heading.get_magnitude();

        //Add code to compute beta for rotating sector & complicated math

        //Place holder for exit point
        let uav_point_exit: Point = Point::new(1f32, 1f32, 1f32);
        let dist_vec: Vector = Vector::from(self.uav_point, uav_point_exit);
        let dist: f32 = dist_vec.get_magnitude();
        let dist_unit_vec: Vector = dist_vec.to_dir();
        let vel_vec: Vector = dist_unit_vec * UAV_SPEED;
        let vel: f32 = vel_vec.get_magnitude();
        // Assuming constant velocity (neglecting drag for now)
        let uav_path_time: f32 = dist / vel;

        //Computes how long the enemy will take to reach uav, considering enemy's heading and
        //bisection of the sector
        let dist_vec_btwn_us: Vector = Vector::from(self.uav_point, self.enemy_point);
        let dist_btwn_us: f32 = dist_vec_btwn_us.get_magnitude();
        let alpha: f32 = (((FoolsMate::THETA / 2f32).sin() / dist) * dist_btwn_us).asin();
        let beta: f32 = 2f32 * std::f32::consts::PI - (FoolsMate::THETA / 2f32) - alpha;
        let dist_vec_to_end: Vector = Vector::from(self.enemy_point, uav_point_exit);
        let dist_to_end: f32 = dist_vec_to_end.get_magnitude();
        let enemy_path: f32 = (dist_to_end / beta.sin()) * alpha.sin();
        let enemy_unit_vec: Vector = self.enemy_heading.to_dir();
        let enemy_dist_vec: Vector = enemy_unit_vec * enemy_path;
        let enemy_dist: f32 = enemy_dist_vec.get_magnitude();
        //Time it takes for the enemy to reach a point (computed by bisecting the sector)
        // on our line of path, assuming constant speed and no drag
        let enemy_path_time: f32 = enemy_dist / ENEMY_SPEED;

        if enemy_path_time > uav_path_time {
            false
        } else {
            true
        }
    }

    //Possible Optimisation: Find closest point on cone and take normal

    //A waypoint above/below the enemy plane and (slightly) within the sphere
    //To account for the enemy moving between checks
    fn evade(&mut self) {
        const HORIZONTAL_INSET: f32 = 0.5f32;
        //2 meters above or below
        let vertical: f32 =
            2f32 * (((self.uav_point.get_z() >= self.enemy_point.get_z()) as i8) * 2) as f32 - 1f32;
        let horizontal: f32 = vertical / FoolsMate::THETA.tan() + HORIZONTAL_INSET;
        let new_waypoint: Point = Point::new(horizontal, horizontal, vertical);
        let first_point: Option<Point> = self.path.pop_front();
        self.path.push_front(new_waypoint);
        match first_point {
            Some(p) => self.path.push_front(p),
            None => (),
        }
    }

    fn evade_close(&mut self) {
        // Find exit point of path through circle
        let r: f32 = self.sphere.get_inner_radius();

        let x1: f32 = self.uav_point.get_x();
        let y1: f32 = self.uav_point.get_y();
        let z1: f32 = self.uav_point.get_z();

        let x: f32 = self.uav_heading.get_i();
        let y: f32 = self.uav_heading.get_j();
        let z: f32 = self.uav_heading.get_k();

        let a: f32 = x * x + y * y + z * z;
        let b: f32 = 2f32 * (x * x1 + y * y1 + z * z1);
        let c: f32 = x1 * x1 + y1 * y1 + z1 * z1 + r * r;
        let disc: f32 = b * b - 4f32 * a * c;
        if disc > 0f32 {
            let sq: f32 = disc.sqrt();
            let exit_lambda: f32 = (-b + sq) / (2f32 * a);
            let half_lambda: f32 = exit_lambda / 2f32;
            let half_point: Point = Point::new(
                x1 + half_lambda * x,
                y1 + half_lambda * y,
                z1 + half_lambda * z,
            );
            let surface: Vector =
                Vector::from(self.enemy_point, half_point).to_dir() * (11f32 / 10f32) * r;
            let surface_point: Point = Point::from_vector(surface);
            let first_point: Option<Point> = self.path.pop_front();
            self.path.push_front(surface_point);
            match first_point {
                Some(p) => self.path.push_front(p),
                None => (),
            }
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
        let uav_point: Point = Point::new(6f32, -6f32, 0f32);
        let enemy_point: Point = Point::new(0f32, 0f32, 0f32);
        let ref_point: Point = Point::new(0f32, 0f32, 0f32);
        let enemy_heading: Vector = Vector::new(1f32, 0f32, 0f32);
        let uav_heading: Vector = Vector::new(0f32, 1f32, 1f32);
        let axis: Vector = Vector::new(0f32, 0f32, 1f32);
        let angle: f32 = std::f32::consts::PI / 4f32;
        let rotation: Quaternion = Quaternion::rotation(angle, axis);
        let sphere: Sphere = Sphere::new(enemy_heading);
        FoolsMate::define(
            ref_point,
            enemy_point,
            uav_point,
            enemy_heading,
            uav_heading,
            path,
            rotation,
            sphere,
        )
    }
    #[test]
    fn test_rotation() {
        let mut fm: FoolsMate = create_fm();
        fm.rotate_space();
        assert_eq!(fm.uav_point, Point::new(6f32, 6f32, 0f32));
        let mut path: LinkedList<Point> = fm.path;
        for i in 0..path.len() {
            let left = path.pop_front();
            let right: Point = Point::new(-1f32 * i as f32, 10f32, 0f32);
            assert_eq!(left, Some(right));
        }
        fm.path = path;
        fm.revert_space();
        assert_eq!(fm.uav_point, Point::new(6f32, -6f32, 0f32));
    }

    #[test]
    fn test_evade() {
        let mut fm: FoolsMate = create_fm();
        let angle: f32 = std::f32::consts::PI / 2f32;
        fm.rotate_space();
        assert_eq!(
            fm.sphere.within_outer_sphere(fm.uav_point, fm.enemy_point),
            true
        );
        assert_eq!(fm.uav_within_sector(angle), true);
        let mut old_path: LinkedList<Point> = LinkedList::new();
        for point in fm.path.iter() {
            old_path.push_back(*point);
        }
        fm.evade();
        assert_ne!(fm.path, old_path);
    }
}
