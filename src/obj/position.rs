// change position to point based!!

use std::fmt;

use obj::location::Location;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    loc: Location,
    time: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loc = ({}, {}, {}) at t = {} \n", self.loc.lat(), self.loc.lon(), self.loc.alt(), self.time)
    }
}

impl Location {
    pub fn to_position(&self, time: f32) -> Position {
        Position {
            loc: *self,
            time,
        }
    }
}

impl Position {
    pub fn new(l: &Location, t: &f32) -> Self {
        Self{
            loc: *l,
            time: *t,
        }
    }

    pub fn loc(&self) -> Location {
        self.loc.into()
    }

    pub fn time(&self) -> f32 {
        self.time.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_position_display() {
        let (lat, lon, alt) = (0.1f32, 0.3f32, 7.0f32);
        let loc = Location::new(lat, lon, alt);
        let time = 0.00123f32;
        let tel = Position::new(&loc, &time);
        print!("{}", tel);
    }

    #[test]
    fn test_location_to_position() {
        let loc = Location::new(0f32, 0f32, 0f32);
        let pos = loc.to_position(3f32);
        assert_eq!(pos.loc(), loc);
        assert_eq!(pos.time(), 3f32);
    }

    #[test]
    fn test_position_new() {
        let (lat, lon, alt) = (0.1f32, 0.3f32, 7.0f32);
        let loc = Location::new(lat, lon, alt);
        let time = 0.00123f32;
        let tel = Position::new(&loc, &time);
        assert_eq!(tel.loc, loc);
        assert_eq!(tel.time, time);
    }

    #[test]
    fn test_position_loc() {
        let (lat, lon, alt) = (0.1f32, 0.3f32, 7.0f32);
        let loc = Location::new(lat, lon, alt);
        let time = 0.00123f32;
        let tel = Position::new(&loc, &time);
        assert_eq!(tel.loc(), tel.loc);
    }

    #[test]
    fn test_position_time() {
        let (lat, lon, alt) = (0.1f32, 0.3f32, 7.0f32);
        let loc = Location::new(lat, lon, alt);
        let time = 0.00123f32;
        let tel = Position::new(&loc, &time);
        assert_eq!(tel.time(), tel.time);
    }
}
