use super::obj::*;

pub mod graph;

use self::graph::*;

struct Careless {
    current: Location,
    target: Location,
    trajectory: Vec<Position>,
    origin: Location,
}

// TO DO: fill in default values
impl Default for Careless {
    fn default() -> Self {
        Self {
            current: Location::new(0f32, 0f32, 0f32),
            target: Location::new(1f32, 1f32, 1f32),
            trajectory: vec![
                Position::new(&Location::new(1f32, 0f32, 0f32), &0f32),
                Position::new(&Location::new(0f32, 1f32, 0f32), &0f32)],
            origin: Location::new(0f32, 0f32, 0f32),

        }
    }
}

impl Careless {
    //fn get_traj_pos(&self) -> Position

    //fn get_traj_vel(&self) -> (f32, f32, f32)

    //fn make_bounds(&self) -> Bound
    // add bound as field of careless?

    //fn check_intersect(&self) -> bool
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_careless_default() {
        let c = Careless::default();
        assert_eq!(c.current, Location::new(0f32, 0f32, 0f32));
        assert_eq!(c.target, Location::new(1f32, 1f32, 1f32));

        assert_eq!(c.trajectory, vec![
            Position::new(&Location::new(1f32, 0f32, 0f32), &0f32),
            Position::new(&Location::new(0f32, 1f32, 0f32), &0f32)]);

        assert_eq!(c.origin, Location::new(0f32, 0f32, 0f32));
    }
}
