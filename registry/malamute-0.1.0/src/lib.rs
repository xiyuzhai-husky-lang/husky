#![allow(warnings, non_snake_case)]
mod narrow_down_;

use std::panic::{RefUnwindSafe, UnwindSafe};

pub use self::narrow_down_::*;

use ad_hoc_devsoul_dependency::{ugly::*, *};
use husky_core::*;

// #[ad_hoc_devsoul_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy, __Serialize)]
pub enum Class<Label> {
    Known(Label),
    Unknown,
}
impl<Label> __Immortal for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    fn try_copy(&self) -> Option<__Value> {
        Some((*self).into_value())
    }
}

impl<Label> __Boiled for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    type Thawed = Class<Label>;
    unsafe fn into_thawed(self) -> Self::Thawed {
        self
    }
}
impl<Label> __Thawed for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    type Frozen = Class<Label>;
    fn freeze(&self) -> Self::Frozen {
        *self
    }

    fn is_copyable() -> bool {
        true
    }

    fn try_copy_thawed(&self) -> Option<__ThawedValue> {
        Some(unsafe { (*self).into_thawed().into_thawed_value() })
    }
}

impl<Label> __Frozen for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    type Thawed = Class<Label>;
    type Slush = ();
    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }

    fn serialize_to_value(&self) -> __JsonValue {
        __to_json_value(self).unwrap()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        __Visual::Void
    }
}

impl<Label> __FromValue for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    fn from_value_aux(value: __Value, _: Option<&mut __SlushValues>) -> Self {
        value.into_owned()
    }
}

impl<Label> __IntoValue for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    fn into_value(self) -> __Value {
        __Value::from_owned(self)
    }
}

impl<Label> __FromThawedValue for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    fn from_thawed_value_aux(value: __ThawedValue, _: Option<&mut __SlushValues>) -> Self {
        value.into_owned()
    }
}

impl<Label> __IntoThawedValue for Class<Label>
where
    Label: __Immortal + Copy + __Serialize,
{
    fn into_thawed_value(self) -> __ThawedValue {
        __ThawedValue::from_owned(self)
    }
}

#[ad_hoc_devsoul_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAll {
    Yes,
    No,
}

#[ad_hoc_devsoul_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAllResult {
    ConfidentYes,
    ConfidentNo,
    Unconfident,
}

impl Default for crate::OneVsAll {
    fn default() -> crate::OneVsAll {
        OneVsAll::No
    }
}

impl<Label> Unveil<crate::OneVsAll> for crate::Class<Label>
where
    Label: Copy,
{
    type RuntimeConstSymbols = (Label,);

    type Output = ();

    fn unveil(
        one_vs_all: crate::OneVsAll,
        (label,): (Label,),
    ) -> husky_core::ops::ControlFlow<crate::Class<Label>, ()> {
        match one_vs_all {
            crate::OneVsAll::Yes => husky_core::ops::ControlFlow::Break(crate::Class::Known(label)),
            crate::OneVsAll::No => husky_core::ops::ControlFlow::Continue(()),
        }
    }
}

impl Unveil<crate::OneVsAllResult> for crate::OneVsAll {
    type RuntimeConstSymbols = ();

    type Output = ();

    fn unveil(
        one_vs_all_result: crate::OneVsAllResult,
        (): (),
    ) -> husky_core::ops::ControlFlow<crate::OneVsAll, ()> {
        match one_vs_all_result {
            crate::OneVsAllResult::ConfidentYes => {
                husky_core::ops::ControlFlow::Break(OneVsAll::Yes)
            }
            crate::OneVsAllResult::ConfidentNo => husky_core::ops::ControlFlow::Break(OneVsAll::No),
            crate::OneVsAllResult::Unconfident => husky_core::ops::ControlFlow::Continue(()),
        }
    }
}
