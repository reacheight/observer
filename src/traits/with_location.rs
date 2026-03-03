use std::{error, fmt};

use crate::types::Location;
use source2_demo::{Entity, try_property};

pub trait WithLocation {
    fn location(&self) -> Result<Location, LocationError>;
    fn try_location(&self) -> Option<Location>;
}

#[derive(Debug)]
pub struct LocationError;

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "can't get location")
    }
}

impl error::Error for LocationError {}

impl WithLocation for Entity {
    fn try_location(&self) -> Option<Location> {
        let get_coord = |coord_literal| {
            let cell: u16 = try_property!(self, "CBodyComponent.m_cell{coord_literal}")?;
            let vec: f32 = try_property!(self, "CBodyComponent.m_vec{coord_literal}")?;

            let coordinate = (cell - 64) as f32 * 128.0 + vec;
            Some(coordinate)
        };

        let x = get_coord("X")?;
        let y = get_coord("Y")?;

        Some(Location::new(x, y))
    }

    fn location(&self) -> Result<Location, LocationError> {
        self.try_location().ok_or(LocationError)
    }
}
