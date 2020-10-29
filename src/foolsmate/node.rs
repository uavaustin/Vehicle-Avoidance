use obj::location::Location;
pub struct Node {
    location: Location,
    modified: bool,
}

impl Node {
    pub fn new(location: Location, modified: bool) -> Self {
        Self {
            location: location,
            modified: modified,
        }
    }
}
