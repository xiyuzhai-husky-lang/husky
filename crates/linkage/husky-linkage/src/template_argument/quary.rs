#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinQuary {
    Ref,
    RefMut,
    Transient,
}
