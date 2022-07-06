use crate::*;

impl<T> Instantiable for Vec<T>
where
    T: Instantiable,
{
    type Target = Vec<T::Target>;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        self.iter().map(|t| t.instantiate(ctx)).collect()
    }
}
