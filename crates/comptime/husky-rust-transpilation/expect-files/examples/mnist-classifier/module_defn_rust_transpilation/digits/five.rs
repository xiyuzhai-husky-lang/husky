use super::*;

#[ad_hoc_task_dependency::val_item(37)]
pub fn is_five() -> OneVsAll {
    OneVsAll::Yes
}