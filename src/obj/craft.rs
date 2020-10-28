use space::point::Point;
use space::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Craft {
    position: Point,
    heading: Vector,
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
