use std::{collections::HashSet, sync::Arc};

use crate::*;

impl<T> Instantiable for Arc<T>
where
    T: Instantiable,
{
    type Target = T::Target;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        (**self).instantiate(ctx)
    }
}
