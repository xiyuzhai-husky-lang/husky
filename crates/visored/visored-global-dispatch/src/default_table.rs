use crate::dispatch::{
    binary_opr::VdBinaryOprGlobalDispatch, separator::VdSeparatorGlobalDispatch,
};
use rustc_hash::FxHashMap;
use visored_opr::{opr::binary::VdBaseBinaryOpr, separator::VdBaseSeparator};
use visored_zfc_ty::ty::VdZfcType;

pub struct VdDefaultGlobalDispatchTable {
    base_binary_opr_default_dispatch_table:
        FxHashMap<VdBaseBinaryOprKey, VdBinaryOprGlobalDispatch>,
    base_separator_default_dispatch_table: FxHashMap<VdBaseSeparatorKey, VdSeparatorGlobalDispatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdBaseBinaryOprKey {
    base_binary_opr: VdBaseBinaryOpr,
    lopd_ty: VdZfcType,
    ropd_ty: VdZfcType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdBaseSeparatorKey {
    base_separator: VdBaseSeparator,
    item_ty: VdZfcType,
}

impl VdDefaultGlobalDispatchTable {
    pub fn new(
        base_binary_opr_default_dispatches: impl IntoIterator<
            Item = (VdBaseBinaryOprKey, VdBinaryOprGlobalDispatch),
        >,
        base_separator_default_dispatches: impl IntoIterator<
            Item = (VdBaseSeparatorKey, VdSeparatorGlobalDispatch),
        >,
    ) -> Self {
        Self {
            base_binary_opr_default_dispatch_table: base_binary_opr_default_dispatches
                .into_iter()
                .collect(),
            base_separator_default_dispatch_table: base_separator_default_dispatches
                .into_iter()
                .collect(),
        }
    }

    pub fn new_standard(db: &::salsa::Db) -> Self {
        Self::new([], [])
    }
}

impl VdDefaultGlobalDispatchTable {
    pub fn base_binary_opr_default_dispatch(
        &self,
        base_binary_opr: VdBaseBinaryOpr,
        lopd_ty: VdZfcType,
        ropd_ty: VdZfcType,
    ) -> Option<&VdBinaryOprGlobalDispatch> {
        self.base_binary_opr_default_dispatch_table
            .get(&VdBaseBinaryOprKey {
                base_binary_opr,
                lopd_ty,
                ropd_ty,
            })
    }

    pub fn base_separator_default_dispatch(
        &self,
        base_separator: VdBaseSeparator,
        item_ty: VdZfcType,
    ) -> Option<&VdSeparatorGlobalDispatch> {
        self.base_separator_default_dispatch_table
            .get(&VdBaseSeparatorKey {
                base_separator,
                item_ty,
            })
    }
}
