
use std::fmt;

use super::position::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    lat: f32,
    lon: f32,
    alt: f32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loc = ({}, {}, {}) \n", self.lat, self.lon, self.alt)
    }
}

impl Location {
    pub fn new(lat: f32, lon: f32, alt: f32) -> Self {
        Self {
            lat,
            lon,
            alt,
        }
    }

    pub fn from_degrees(latd: f32, lond: f32, alt: f32) -> Self {
        Self {
            lat: latd.to_radians().into(),
            lon: lond.to_radians().into(),
            alt: alt.into(),
        }
    }

    pub fn lat(&self) -> f32 {
        self.lat.into()
    }

    pub fn lon(&self) -> f32 {
        self.lon.into()
    }

    pub fn alt(&self) -> f32 {
        self.alt.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_location_display() {
        let (lat, lon, alt) = (1f32, 1f32, 1f32);
        let l = Location::new(lat, lon, alt);
        print!("{}", l);
    }

    #[test]
    fn test_location_new() {
        let (lat, lon, alt) = (0.1f32, 0.3f32, 7.0f32);
        let loc = Location::new(lat, lon, alt);
        assert!(loc.lat() - lat < 0.1 );
        assert!(loc.lon() - lon < 0.1 );
        assert!(loc.alt() - alt < 0.1 );
    }
}
