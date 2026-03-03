use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
