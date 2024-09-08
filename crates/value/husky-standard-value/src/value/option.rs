use super::*;

impl<T> Immortal for Option<T>
where
    T: Immortal,
{
    fn try_copy(&self) -> Option<Value> {
        if !Self::is_copyable() {
            return None;
        }
        self.as_ref().map(|v| v.try_copy().unwrap())
    }

    fn unwrap_leash(&'static self) -> ExceptedValue {
        let slf = self.as_ref().ok_or(Exception::UnwrapNone)?;
        Ok(match slf.try_copy() {
            Some(slf) => slf,
            None => Value::from_leash(slf),
        })
    }
}
