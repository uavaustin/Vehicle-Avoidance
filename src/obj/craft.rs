use foolsmate::space::point::Point;
use foolsmate::space::vector::Vector;
use obj::location::Location;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Craft {
    location: Location,
    heading: Vector,
}

impl fmt::Display for Craft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Craft: Location @ {}, Heading: ({}, {}, {})\n",
            self.location,
            self.heading.get_i(),
            self.heading.get_j(),
            self.heading.get_k()
        )
    }
}

impl Craft {
    pub fn new(location: Location, heading: Vector) -> Self {
        Self {
            location: location,
            heading: heading,
        }
    }

    pub fn from_locations(current_loc: Location, prev_loc: Location) -> Self {
        let ref_point = Point::define_ref(prev_loc);
        let second_point = Point::from_location(current_loc, ref_point);
        let heading = Vector::from(ref_point, second_point);
        Self {
            location: current_loc,
            heading: heading,
        }
    }

    pub fn get_location(&self) -> Location {
        self.location
    }

    pub fn get_heading(&self) -> Vector {
        self.heading
    }
}
