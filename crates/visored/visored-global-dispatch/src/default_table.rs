use crate::{
    dispatch::{
        attach::VdAttachGlobalDispatch, binary_opr::VdBinaryOprGlobalDispatch,
        separator::VdSeparatorGlobalDispatch,
    },
    menu::vd_global_dispatch_menu,
};
use rustc_hash::FxHashMap;
use visored_opr::{menu::vd_opr_menu, opr::binary::VdBaseBinaryOpr, separator::VdBaseSeparator};
use visored_signature::menu::vd_signature_menu;
use visored_term::{menu::vd_ty_menu, ty::VdType};

pub struct VdDefaultGlobalDispatchTable {
    base_binary_opr_default_dispatch_table:
        FxHashMap<VdBaseBinaryOprKey, VdBinaryOprGlobalDispatch>,
    base_separator_default_dispatch_table: FxHashMap<VdBaseSeparatorKey, VdSeparatorGlobalDispatch>,
    attach_default_dispatch_table: FxHashMap<VdAttachKey, VdAttachGlobalDispatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdBaseBinaryOprKey {
    lopd_ty: VdType,
    base_binary_opr: VdBaseBinaryOpr,
    ropd_ty: VdType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdBaseSeparatorKey {
    base_separator: VdBaseSeparator,
    prev_item_ty: VdType,
    next_item_ty: VdType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdAttachKey {
    Power {
        base_ty: VdType,
        exponent_ty: VdType,
    },
}

impl VdDefaultGlobalDispatchTable {
    pub fn new(
        base_binary_opr_default_dispatches: impl IntoIterator<
            Item = (VdBaseBinaryOprKey, VdBinaryOprGlobalDispatch),
        >,
        base_separator_default_dispatches: impl IntoIterator<
            Item = (VdBaseSeparatorKey, VdSeparatorGlobalDispatch),
        >,
        attach_default_dispatches: impl IntoIterator<Item = (VdAttachKey, VdAttachGlobalDispatch)>,
    ) -> Self {
        Self {
            base_binary_opr_default_dispatch_table: base_binary_opr_default_dispatches
                .into_iter()
                .collect(),
            base_separator_default_dispatch_table: base_separator_default_dispatches
                .into_iter()
                .collect(),
            attach_default_dispatch_table: attach_default_dispatches.into_iter().collect(),
        }
    }

    pub fn new_standard(db: &::salsa::Db) -> Self {
        let zfc_ty_menu = vd_ty_menu(db);
        let opr_menu = vd_opr_menu(db);
        let global_dispatch_menu = vd_global_dispatch_menu(db);
        Self::new(
            VdBinaryOprGlobalDispatch::standard_defaults(
                zfc_ty_menu,
                opr_menu,
                global_dispatch_menu,
            )
            .into_iter()
            .map(|((lopd_ty, base_binary_opr, ropd_ty), dispatch)| {
                (
                    VdBaseBinaryOprKey {
                        lopd_ty,
                        base_binary_opr,
                        ropd_ty,
                    }
                    .into(),
                    dispatch,
                )
            }),
            VdSeparatorGlobalDispatch::standard_defaults(
                zfc_ty_menu,
                opr_menu,
                global_dispatch_menu,
            )
            .into_iter()
            .map(|((prev_item_ty, base_separator, next_item_ty), dispatch)| {
                (
                    VdBaseSeparatorKey {
                        prev_item_ty,
                        base_separator,
                        next_item_ty,
                    }
                    .into(),
                    dispatch,
                )
            }),
            VdAttachGlobalDispatch::standard_defaults(zfc_ty_menu, global_dispatch_menu)
                .into_iter()
                .map(|(key, dispatch)| (key.into(), dispatch)),
        )
    }
}

impl VdDefaultGlobalDispatchTable {
    pub fn base_binary_opr_default_dispatch(
        &self,
        base_binary_opr: VdBaseBinaryOpr,
        lopd_ty: VdType,
        ropd_ty: VdType,
    ) -> Option<VdBinaryOprGlobalDispatch> {
        self.base_binary_opr_default_dispatch_table
            .get(&VdBaseBinaryOprKey {
                base_binary_opr,
                lopd_ty,
                ropd_ty,
            })
            .copied()
    }

    pub fn base_separator_default_dispatch(
        &self,
        base_separator: VdBaseSeparator,
        prev_item_ty: VdType,
        next_item_ty: VdType,
    ) -> Option<VdSeparatorGlobalDispatch> {
        self.base_separator_default_dispatch_table
            .get(&VdBaseSeparatorKey {
                base_separator,
                prev_item_ty,
                next_item_ty,
            })
            .copied()
    }

    pub fn power_default_dispatch(
        &self,
        base_ty: VdType,
        exponent_ty: VdType,
    ) -> Option<VdAttachGlobalDispatch> {
        self.attach_default_dispatch_table
            .get(&VdAttachKey::Power {
                base_ty,
                exponent_ty,
            })
            .copied()
    }
}
