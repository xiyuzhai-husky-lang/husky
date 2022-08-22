use crate::*;

pub trait __WithEvalLifetime<'eval> {
    type This;
}

pub trait __Any: __StaticInfo + for<'eval> __WithEvalLifetime<'eval> {}
