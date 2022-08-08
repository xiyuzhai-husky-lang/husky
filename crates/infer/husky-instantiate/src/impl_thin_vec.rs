use crate::*;

impl<T> Instantiable for ThinVec<T>
where
    T: Instantiable,
{
    type Target = ThinVec<T::Target>;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        self.iter().map(|t| t.instantiate(ctx)).collect()
    }
}
