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

//Returns the triangle's width, the point where the UAV enters the arc, and the point where the UAV exits
fn slice(uav: UAV, enemy: Enemy) -> (f32, Point, Point) {

    let uav_location:Point = uav.get_location();
    let next_node:Point = uav.get_path()[1];
    let center:Point = enemy.get_location();

    //Orthogonal direction vector of the plane
    let dir:Vector = Vector::from(center, next_node) * Vector::from(center, uav_location);
    //Flat plane: z = center.get_z()
    let z_flat:Vector = Vector::new(0f32, 0f32, 1f32);

    //X-axis: x = 0
    let x_flat:Vector = Vector::new(1f32, 0f32, 0f32);


    //Finding the angle between the plane defining the path and the circle and the x and y axis
    let delta:f32 = z_flat.angle(dir);

    //Offset off the heading
    let alpha:f32 = CONE_THETA * (delta * f32::consts::PI / CONE_THETA).cos() / 2f32;

    //Offset from the x-axis
    let gamma:f32 = x_flat.angle(dir);

    //Sector is delta degrees above/below the horizontal with angle alpha facing in the heading of the enemy
    //NOTE: the plane containing hte path and the center is not necessarily rotated in the same direction as the enemy


}
