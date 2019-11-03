// metric version of position

use std::fmt;

use obj::position::Position;

use careless::graph::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Track {
    point: Point,
    time: f32,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loc = ({}, {}, {}) at t = {} \n", self.point.x(), self.point.y(), self.point.z(), self.time)
    }
}

impl Point {
    pub fn to_track(&self, time: f32) -> Track {
        Track {
            point: *self,
            time,
        }
    }
}

impl Track {
    pub fn new(p: &Point, t: &f32) -> Self {
        Self{
            point: *p,
            time: *t,
        }
    }

    pub fn point(&self) -> Point {
        self.point.into()
    }

    pub fn time(&self) -> f32 {
        self.time.into()
    }


}
