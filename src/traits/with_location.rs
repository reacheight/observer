use crate::types::Location;
use source2_demo::{
    Entity,
    error::{EntityError, FieldValueError},
    property,
};

pub trait WithLocation {
    fn location(&self) -> Result<Location, LocationError>;
}

impl WithLocation for Entity {
    fn location(&self) -> Result<Location, LocationError> {
        let get_coord = |coord_literal: &str| -> Result<f32, LocationError> {
            let cell: u16 = property!(self, "CBodyComponent.m_cell{coord_literal}");
            let vec: f32 = property!(self, "CBodyComponent.m_vec{coord_literal}");

            let coordinate = (cell - 64) as f32 * 128.0 + vec;
            Ok(coordinate)
        };

        let x = get_coord("X")?;
        let y = get_coord("Y")?;

        Ok(Location { x, y })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LocationError {
    #[error(transparent)]
    EntityError(#[from] EntityError),

    #[error(transparent)]
    PropertyValueError(#[from] FieldValueError),
}
