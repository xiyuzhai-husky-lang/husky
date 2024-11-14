use super::*;
use crate::{default_table::VdBaseBinaryOprKey, menu::VdGlobalDispatchMenu};
use visored_opr::{menu::VdOprMenu, opr::binary::VdBaseBinaryOpr};
use visored_zfc_ty::{menu::VdZfcTypeMenu, ty::VdZfcType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VdBinaryOprGlobalDispatch {}

impl VdBinaryOprGlobalDispatch {
    pub fn standard_defaults<'db>(
        zfc_ty_menu: &'db VdZfcTypeMenu,
        vd_opr_menu: &'db VdOprMenu,
        global_dispatch_menu: &'db VdGlobalDispatchMenu,
    ) -> impl IntoIterator<
        Item = (
            (VdZfcType, VdBaseBinaryOpr, VdZfcType),
            &'db VdBinaryOprGlobalDispatch,
        ),
    > {
        []
    }
}
