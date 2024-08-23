//! ad hoc
use husky_standard_devsoul_interface::ugly::*;

#[allow(non_upper_case_globals)]
pub static mut __TASK__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

pub struct TASK;

impl __IsStaticVar<__StaticVarId> for TASK {
    fn item_path_id_interface() -> __ItemPathIdInterface {
        todo!()
    }

    fn ids(locked: &[__ItemPathIdInterface]) -> impl Iterator<Item = __StaticVarId> {
        [].into_iter()
    }

    fn get_id() -> __StaticVarId {
        todo!()
    }

    unsafe fn replace_id(id: __StaticVarId) -> Option<__StaticVarId> {
        todo!()
    }
}
