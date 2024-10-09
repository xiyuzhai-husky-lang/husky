use super::*;
use std::ops::ControlFlow;

impl<C, B> Thawed for ControlFlow<C, B>
where
    C: Thawed,
    B: Thawed,
{
    type Frozen = ControlFlow<C::Frozen, B::Frozen>;

    fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn is_copyable() -> bool {
        todo!()
    }

    fn try_copy_thawed(&self) -> Option<super::ThawedValue> {
        todo!()
    }
}
