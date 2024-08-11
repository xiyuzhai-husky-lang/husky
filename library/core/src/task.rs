//! ad hoc

use husky_devsoul_interface::ugly::__ItemPathIdInterface;
use husky_standard_devsoul_interface::ugly::__StaticVarId;

#[allow(non_upper_case_globals)]
pub static mut __TASK__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

pub struct TASK;

impl TASK {
    pub fn set_up_for_testing(index: usize) {
        todo!()
    }

    pub fn get_id() -> __StaticVarId {
        todo!()
    }

    pub unsafe fn replace_id(id: __StaticVarId) -> Option<__StaticVarId> {
        todo!()
    }

    pub fn ids() -> impl Iterator<Item = __StaticVarId> {
        [].into_iter()
    }
}
