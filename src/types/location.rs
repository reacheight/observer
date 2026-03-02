use std::fmt;

#[derive(Debug)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
