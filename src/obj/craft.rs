use space::point::Point;
use space::vector::Vector;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Craft {
    position: Point,
    heading: Vector,
}

impl fmt::Display for Craft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Craft: Position @ {}, Heading: ({}, {}, {})\n",
            self.position,
            self.heading.get_i(),
            self.heading.get_j(),
            self.heading.get_k()
        )
    }
}

impl Craft {
    pub fn new(position: Point, heading: Vector) -> Self {
        Self {
            position: position,
            heading: heading,
        }
    }

    pub fn from_positions(current_pos: Point, prev_pos: Point) -> Self {
        let heading = Vector::from(current_pos, prev_pos);
        Self {
            position: current_pos,
            heading: heading,
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn heading(&self) -> Vector {
        self.heading
    }
}
