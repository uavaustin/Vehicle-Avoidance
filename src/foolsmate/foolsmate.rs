use super::point::Point;
use super::point::Point2D;
use super::enemy::Enemy;
use super::uav::UAV;
use super::vector::Vector;
use std::f32;

//Taking distance in 10 seconds (assuming usits of speed are /s)
const TIME:f32 = 10f32;

// pi/4 In radians
const CONE_THETA:f32 = f32::consts::PI / 4;

//Make appropriate methods private eventually
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
fn slice(uav: UAV, enemy: Enemy) -> (f32, Point, Point) {

    let uav_location:Point = uav.get_location();
    let next_node:Point = uav.get_path()[1];
    let center:Point = enemy.get_location();

    let dir:Vector = Vector::from(uav_location, next_node).cross(Vector::from(uav_location, center));
    let z_flat:Point = Point::new(0f32, 0f32, center.get_z());

    //Assumes enemy is travelling horizontally
    let flat_plane_1:Vector = Vector::new(Point::new(center.get_x()+(enemy.to_degrees() - CONE_THETA).cos(),
     center.get_y()+(enemy.to_degrees() - CONE_THETA).sin(), center.get_z()), z_flat);
    let flat_plane_2:Vector = Vector::new(Point::new(center.get_x()+(enemy.to_degrees() + CONE_THETA).cos(),
     center.get_y()+(enemy.to_degrees() + CONE_THETA).sin(), center.get_z(), z_flat);
    let flat_plane:Vector = Vector::from(first_plane_1, );

    let alpha:f32 = CONE_THETA * (delta * f32::consts::PI / CONE_THETA).cos();

}
