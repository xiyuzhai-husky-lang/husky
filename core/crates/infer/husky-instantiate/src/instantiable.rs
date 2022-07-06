use crate::*;

pub trait Instantiable {
    type Target;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target;
}
