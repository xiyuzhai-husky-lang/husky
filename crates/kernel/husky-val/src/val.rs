#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Val {
    data: ValData,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ValData {}
