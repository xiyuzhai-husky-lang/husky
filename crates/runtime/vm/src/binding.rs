#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Binding {
    Ref,
    RefMut,
    Move,
    Copy,
}
