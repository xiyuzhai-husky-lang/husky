use super::*;

impl<T> Immortal for Option<T>
where
    T: Immortal,
{
    fn is_copyable() -> bool {
        T::is_copyable()
    }

    fn try_copy(&self) -> Option<Value> {
        if !Self::is_copyable() {
            return None;
        }
        self.as_ref().map(|v| v.try_copy().unwrap())
    }

    fn is_some(&self) -> bool {
        self.is_some()
    }
    fn is_none(&self) -> bool {
        self.is_none()
    }

    fn unwrap_leash(&'static self) -> ExceptedValue {
        let slf = self.as_ref().ok_or(Exception::UnwrapNone)?;
        Ok(match slf.try_copy() {
            Some(slf) => slf,
            None => Value::from_leash(slf),
        })
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        self.as_ref()
            .map(|slf| slf.serialize_to_value())
            .unwrap_or_default()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}
