#![allow(dead_code)]
#![allow(unused_imports)]

// inputs: plane telem, our path

// checks for intersection between plane_bound and path

mod obj;
mod careless;

use obj::position::{Position};
use obj::location::{Location};
use careless::graph::point::{Point};

struct Avoider {
    now: &'static Vec<Position>,
}

/*
impl Avoider {
    fn from_vec(now: &Vec<Position>) -> Self {
        Self {
            now,
        }
    }

    //fn get_velocity() -> float {
    //
    //}

    //fn check_intersection() -> {
    //
    //}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_avoidance_from_vec() {
        let p1 = Position::new(&Location::new(0f32, 0f32, 0f32), &0f32);
        let p2 = Position::new(&Location::new(1f32, 1f32, 1f32), &1f32);
        let p3 = Position::new(&Location::new(3f32, 3f32, 3f32), &2f32);
        let v = vec![p1, p2, p3];
        let A = Avoider::from_vec(&v);
    }
}
*/
