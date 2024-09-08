use super::*;

impl<T> Thawed for Option<T>
where
    T: Thawed,
{
    type Frozen = Option<T::Frozen>;

    fn is_copyable() -> bool {
        T::is_copyable()
    }

    fn try_copy_thawed(&self) -> Option<ThawedValue> {
        if !Self::is_copyable() {
            return None;
        }
        self.as_ref().map(|v| v.try_copy_thawed().unwrap())
    }

    fn is_some(&self) -> bool {
        self.is_some()
    }
    fn is_none(&self) -> bool {
        self.is_none()
    }

    fn unwrap_ref_thawed<'a>(&'a self) -> ExceptedThawedValue {
        let slf = self.as_ref().ok_or(Exception::UnwrapNone)?;
        Ok(match slf.try_copy_thawed() {
            Some(_) => todo!(),
            None => todo!(),
        })
    }

    fn unwrap_leash_thawed(&'static self) -> ExceptedThawedValue {
        let slf = self.as_ref().ok_or(Exception::UnwrapNone)?;
        Ok(match slf.try_copy_thawed() {
            Some(slf) => slf,
            None => ThawedValue::from_leash(slf),
        })
    }

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
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
