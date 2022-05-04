#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Binding {
    Pure,
    BorrowMut,
    Move,
}
