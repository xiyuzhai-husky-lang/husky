#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Binding {
    Ref,
    RefMut,
    Move,
    Copy,
}
