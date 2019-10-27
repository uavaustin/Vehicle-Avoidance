
use obj::location::Location;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    loc: Location,
    time: f32,
}

impl Position {
    pub fn new(l: &Location, t: &f32) -> Self{
        Self{
            loc: *l,
            time: *t,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_position_new() {
        let (lat, lon, alt) = (0.1f32, 0.3f32, 7.0f32);
        let loc = Location::new(lat, lon, alt);
        let time = 0.00123f32;
        let tel = Position::new(&loc, &time);
        assert_eq!(tel.loc, loc);
        assert_eq!(tel.time, time);
    }
}
