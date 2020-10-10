use super::point::Point;
use super::enemy::Enemy;
use super::uav::UAV;

//Make private eventually
pub fn within_sphere(enemy_location:Point, uav_location:Point) -> bool {
    //Taking distance in 10 seconds (assuming usits of speed are /s)
    const TIME:f32 = 10f32;
    euclidean_dist(uav_location, enemy_location) <= radius * TIME
}

pub fn euclidean_dist(first_point:Point, second_point:Point) -> f32 {
    ((first_point.get_x().powi(2) - second_point.get_x().powi(2)) +
        (first_point.get_y().powi(2) - second_point.get_y().powi(2)) +
        (first_point.get_z().powi(2) - second_point.get_z().powi(2))).sqrt()
}

//Returns theta, the point where the UAV enters the arc, and the point where the UAV exits
fn slice(uav: UAV, enemy: Enemy) -> (f32, Point, Point) {

}
