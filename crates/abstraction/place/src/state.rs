#[derive(Debug)]
pub enum PlaceState<T> {
    Occupied(T),
    Empty,
}

impl<T> Default for PlaceState<T> {
    fn default() -> Self {
        PlaceState::Empty
    }
}
