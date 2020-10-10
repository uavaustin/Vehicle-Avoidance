use super::point::Point;
use super::point::Point2D;
use super::enemy::Enemy;
use super::uav::UAV;

//Taking distance in 10 seconds (assuming usits of speed are /s)
const TIME:f32 = 10f32;

//Make private eventually
pub fn within_sphere(enemy_location:Point, uav:UAV) -> bool {
    const RADIUS = TIME * uav.get_speed();
    let uav_location = uav.get_location();
    euclidean_dist(uav_location, enemy_location) <= RADIUS;
}

pub fn euclidean_dist(first_point:Point, second_point:Point) -> f32 {
    ((first_point.get_x().powi(2) - second_point.get_x().powi(2)) +
        (first_point.get_y().powi(2) - second_point.get_y().powi(2)) +
        (first_point.get_z().powi(2) - second_point.get_z().powi(2))).sqrt()
}

//Returns the triangle's theta, the point where the UAV enters the arc, and the point where the UAV exits
fn slice(theta: f32, uav: UAV, enemy: Enemy) -> (f32, Point2D, Point2D) {



}
