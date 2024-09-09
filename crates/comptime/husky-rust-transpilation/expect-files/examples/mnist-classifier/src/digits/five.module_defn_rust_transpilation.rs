use super::*;

#[allow(non_upper_case_globals)]
pub static mut __is_five__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __is_five__ITEM_PATH_ID_INTERFACE,
    var_deps = []
)]
pub fn is_five() -> malamute::OneVsAll {
    OneVsAll::Yes
}