use super::*;

impl<T> Immortal for Vec<T>
where
    T: Immortal,
{
    fn try_copy(&self) -> Option<Value> {
        None
    }

    fn index_leash(&'static self, index: usize) -> ExceptedValue {
        Ok(Value::from_leash(&self[index]))
    }
}
