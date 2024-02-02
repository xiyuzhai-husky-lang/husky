#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinPlace {
    Ref,
    RefMut,
    Transient,
}
