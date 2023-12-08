use super::*;

#[ad_hoc_task_dependency::val_item]
pub fn is_five() -> OneVsAll {
    OneVsAll::Yes
}