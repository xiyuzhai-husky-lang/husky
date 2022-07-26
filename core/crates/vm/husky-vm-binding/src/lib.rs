#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Binding {
    EvalRef,
    TempRef,
    TempMut,
    Move,
    Copy,
}
