#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinkagePlace {
    Ref,
    RefMut,
    Transient,
}
