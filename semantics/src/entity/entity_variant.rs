use common::*;

#[derive(Debug, Clone, Copy)]
pub enum ScopeVariant {
    Opn(Opn),
    Type(Type),
    Module,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opn {
    Main,
    MemFunc,
    MemLazy,
    MemVar,
    Func,
    Pattern,
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Struct,
    Rename,
    Enum,
    Primitive,
    Vector,
    Array,
}
