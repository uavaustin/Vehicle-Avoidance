use foolsmate::space::point::Point;
use foolsmate::space::vector::Vector;

pub struct Sphere {
    inner_radius: f32,
    outer_radius: f32,
}

impl Sphere {
    pub fn new(enemy_speed: Vector) -> Self {
        const OUTER_TIME_IN_SECS: f32 = 30f32;
        const INNER_TIME_IN_SECS: f32 = 10f32;
        let speed: f32 = enemy_speed.get_magnitude();
        Sphere {
            inner_radius: speed * INNER_TIME_IN_SECS,
            outer_radius: speed * OUTER_TIME_IN_SECS,
        }
    }

    fn within_inner_sphere(&self, uav_point: Point, enemy_point: Point) -> bool {
        uav_point.dist(enemy_point) <= self.inner_radius
    }

    fn within_outer_sphere(&self, uav_point: Point, enemy_point: Point) -> bool {
        uav_point.dist(enemy_point) <= self.outer_radius
    }
}
