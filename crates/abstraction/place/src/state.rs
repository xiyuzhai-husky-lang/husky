#[derive(Debug, Clone)]
pub enum PlaceState<T> {
    Occupied(T),
    Uninitialized,
    Destroyed,
}

impl<T> Default for PlaceState<T> {
    fn default() -> Self {
        PlaceState::Uninitialized
    }
}
