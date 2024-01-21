#![allow(warnings, non_snake_case)]
mod narrow_down_;

pub use self::narrow_down_::*;

use ad_hoc_task_dependency::{ugly::*, *};
use husky_core::*;

ad_hoc_task_dependency::init_crate!();

// #[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy, __Serialize)]
pub enum Class<Label> {
    Known(Label),
    Unknown,
}
impl<Label> __WeakStatic for Class<Label>
where
    Label: __WeakStatic<Static = Label> + __Static + __Serialize + Copy,
{
    type Static = Class<<Label as __WeakStatic>::Static>;
    unsafe fn into_static(self) -> Self::Static {
        self
    }
}
impl<Label> __Static for Class<Label>
where
    Label: __WeakStatic<Static = Label> + __Static + __Serialize + Copy,
{
    type Frozen = Class<Label>;
    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn copy(&self) -> Box<dyn __StaticDyn> {
        Box::<Class<Label>>::new(self.clone())
    }

    fn serialize_to_value(&self) -> __JsonValue {
        __to_json_value(self).unwrap()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        __Visual::Void
    }
}

impl<Label> __Frozen for Class<Label>
where
    Label: __WeakStatic<Static = Label> + __Static + __Serialize + Copy,
{
    type Static = Class<Label>;
    type Stand = ();
    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}
impl<Label> __FromValue for Class<Label>
where
    Label: __WeakStatic<Static = Label> + __Static + Copy,
{
    fn from_value_aux(value: __Value, _: Option<&mut __ValueStands>) -> Self {
        value.into_owned()
    }
}
impl<Label> __IntoValue for Class<Label>
where
    Label: __WeakStatic<Static = Label> + __Static + __Serialize + Copy,
{
    fn into_value(self) -> __Value {
        __Value::from_owned(self)
    }
}

#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAll {
    Yes,
    No,
}

#[ad_hoc_task_dependency::value_conversion]
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
