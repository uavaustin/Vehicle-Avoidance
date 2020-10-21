use super::point::Point;
use super::point::Point2D;
use super::enemy::Enemy;
use super::uav::UAV;
use super::vector::Vector;
use super::quaternion::Quaternion;
use std::f32;

//Taking distance in 10 seconds (assuming usits of speed are /s)
const TIME:f32 = 10f32;

// pi/4 In radians
const CONE_THETA:f32 = f32::consts::PI / 4;

pub 

//Make appropriate methods private eventually
pub fn within_sphere(enemy_location:Point, uav:UAV) -> bool {
    const RADIUS:f32 = TIME * uav.get_speed();
    let uav_location = uav.get_location();
    euclidean_dist(uav_location, enemy_location) <= RADIUS;
}

pub fn euclidean_dist(first_point:Point, second_point:Point) -> f32 {
    ((first_point.get_x().powi(2) - second_point.get_x().powi(2)) +
        (first_point.get_y().powi(2) - second_point.get_y().powi(2)) +
        (first_point.get_z().powi(2) - second_point.get_z().powi(2))).sqrt()
}

//Returns the triangle's width, the point where the UAV enters the arc, and the point where the UAV exits
pub fn slice(uav: UAV, enemy: Enemy) -> (f32, Point, Point) {
    const RADIUS:f32 = TIME * uav.get_speed();

    let uav_location:Point = uav.get_location();

    //Assumes nodes are removed from the list after reaching them
    let next_node:Point = uav.get_path()[0];
    let center:Point = enemy.get_location();

    //Orthogonal direction vector of the plane
    let dir:Vector = Vector::from(center, next_node) * Vector::from(center, uav_location);

    let mut enemy_dir:Vector = Vector::new(1f32, 0f32, 0f32);
    let mut enemy_perp:Vector = Vector::new(0f32,1f32,0f32);

    let verical_vector:Vector = Vector::new(0f32,0f32,1f32);

    let horizontal_rotation:Quaternion = Quaternion::new(enemy.to_degrees().to_radians(), vertical_vector);

    enemy_dir = horizontal_rotation.rotate(enemy_dir);
    enemy_perp = horizontal_rotation.rotate(enemy_perp);

    let vertical_rotation_vector:Vector = Vector::new(enemy.to_degrees().to_radians().cos(), enemy.to_degrees().to_radians().sin(), 0) * vertical_vector;

    let vertical_rotation:Quaternion = Quaternion::new(enemy.get_pitch(), vertical_rotation_vector);

    //Now have a plane containing the projected straight enemy path
    enemy_dir = veretical_rotation.rotate(enemy_dir);
    enemy_perp = vertical_rotation.rotate(enemy_perp);

}

pub fn find
