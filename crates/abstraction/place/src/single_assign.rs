use crate::*;
use error::*;

#[derive(Debug, Clone)]
pub struct SingleAssignPlace<T>(PlaceState<T>);

impl<T> Default for SingleAssignPlace<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> SingleAssignPlace<T> {
    pub fn set(&mut self, t: T) -> SingleAssignResult<()> {
        match self.0 {
            PlaceState::Occupied(_) => Err(SingleAssignError::SetOccupiedValue),
            PlaceState::Uninitialized => Ok(self.0 = PlaceState::Occupied(t)),
            PlaceState::Destroyed => todo!(),
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self.0 {
            PlaceState::Occupied(ref t) => Some(t),
            PlaceState::Uninitialized => None,
            PlaceState::Destroyed => todo!(),
        }
    }
}

mod error {
    use thiserror::Error;

    pub type SingleAssignResult<T> = Result<T, SingleAssignError>;

    #[derive(Debug, Error)]
    pub enum SingleAssignError {
        #[error("set occupied value")]
        SetOccupiedValue,
    }
}

mod tests {
    // todo: write tests
}
