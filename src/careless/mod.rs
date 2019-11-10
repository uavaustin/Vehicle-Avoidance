
use super::obj::*;

pub mod graph;

use self::graph::point::Point;
use self::graph::track::Track;


struct Careless {
    current: Point,
    target: Point,
    trajectory: Vec<Track>,
    safe_time: f32,
    bound: f32,
}

// TO DO: fill in default values
impl Default for Careless {
    fn default() -> Self {
        Self {
            current: Point::new(0f32, 0f32, 0f32),
            target: Point::new(1f32, 1f32, 1f32),
            trajectory: vec![
                Track::new(&Point::new(1f32, 0f32, 0f32), &0f32),
                Track::new(&Point::new(0f32, 1f32, 0f32), &0f32)],
            safe_time: 10f32,
            bound: 0f32,
        }
    }
}

impl Careless {

    fn get_safety(&self) -> f32 {
        self.safe_time.into()
    }

    //need to choose proper method for determining vel
    fn get_traj_vel(&self) -> f32 {
        // for now get most recently velocity
        let last = self.trajectory.len();
        let x1 = self.trajectory[last].point();
        let x2 = self.trajectory[last - 1].point();
        let delta_t = self.trajectory[last].time() - self.trajectory[last - 1].time();
        let d = x1.dist(&x2);
        d / delta_t
    }

    // for now just use radial output
    fn set_bound(mut self, vel: f32) {
        self.bound = vel * self.get_safety();
    }

    fn check_intersection(&self) -> bool {
        let a = self.current;
        let b = self.target;
        let c = self.trajectory.last().unwrap().point();

        let d = (c.x() * (a.y() - b.y()) - c.y() * (a.x() - b.x()) + b.x() * a.y() + b.y() * a.x()).abs()
            / a.dist(&b);

        if d > self.bound {
            false
        } else {
            true
        }
    }

    fn set_track(&mut self, plane: Vec<Track>) {
        self.trajectory = plane;
         = self.get_traj_vel;
        /*
        Self {
            current: self.current,
            target: self.target,
            trajectory: vec![p1, p2],
            safe_time: 10f32,
            bound: 0f32,
        }
        */
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_careless_default() {
        let c = Careless::default();
        assert_eq!(c.current, Point::new(0f32, 0f32, 0f32));
        assert_eq!(c.target, Point::new(1f32, 1f32, 1f32));

        assert_eq!(c.trajectory, vec![
            Track::new(&Point::new(1f32, 0f32, 0f32), &0f32),
            Track::new(&Point::new(0f32, 1f32, 0f32), &0f32)]);
        assert_eq!(c.safe_time, 10f32);
        assert_eq!(c.bound, 0f32);
    }

    #[test]
    fn test_get_safety() {

    }
}
