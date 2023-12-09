use super::*;

#[ad_hoc_task_dependency::val_item(ingredient_index = 37)]
pub fn is_five() -> OneVsAll {
    OneVsAll::Yes
}