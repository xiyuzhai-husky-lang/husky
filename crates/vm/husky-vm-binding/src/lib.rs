#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Binding {
    Leash,
    TempRef,
    TempMut,
    Move,
    Copy,
    DerefCopy,
}
