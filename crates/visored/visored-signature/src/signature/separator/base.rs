use visored_entity_path::path::{trai_item::VdTraitItemPath, VdItemPath};
use visored_opr::separator::VdBaseSeparator;

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct VdBaseSeparatorSignature {
    instantiation: VdInstantiation,
    // TODO: replace this with something more ethereal
    opr: VdBaseSeparator,
    item_ty: VdType,
    expr_ty: VdType,
}

impl From<VdBaseSeparatorSignature> for VdSignature {
    fn from(signature: VdBaseSeparatorSignature) -> Self {
        VdSignature::Separator(signature.into())
    }
}

impl VdBaseSeparatorSignature {
    pub fn new(instantiation: VdInstantiation, item_ty: VdType, expr_ty: VdType) -> Self {
        let VdItemPath::TraitItem(path) = instantiation.path() else {
            unreachable!()
        };
        let opr = match path {
            VdTraitItemPath::GroupMul => todo!(),
            VdTraitItemPath::AbelianGroupAdd => todo!(),
            VdTraitItemPath::NatAdd => VdBaseSeparator::ADD,
            VdTraitItemPath::NatMul => VdBaseSeparator::MUL,
            VdTraitItemPath::RingAdd => VdBaseSeparator::ADD,
            VdTraitItemPath::RingSub => todo!(),
            VdTraitItemPath::RingMul => VdBaseSeparator::MUL,
            VdTraitItemPath::RingPower => todo!(),
            VdTraitItemPath::RingPos => todo!(),
            VdTraitItemPath::RingNeg => todo!(),
            VdTraitItemPath::Eq => VdBaseSeparator::EQ,
            VdTraitItemPath::Ne => VdBaseSeparator::NE,
            VdTraitItemPath::Lt => VdBaseSeparator::LT,
            VdTraitItemPath::Gt => VdBaseSeparator::GT,
            VdTraitItemPath::Le => VdBaseSeparator::LE,
            VdTraitItemPath::Ge => VdBaseSeparator::GE,
            VdTraitItemPath::FieldDiv => todo!(),
            VdTraitItemPath::RealSqrt => todo!(),
        };
        Self {
            instantiation,
            opr,
            item_ty,
            expr_ty,
        }
    }
}

impl VdBaseSeparatorSignature {
    pub fn instantiation(self) -> VdInstantiation {
        self.instantiation
    }

    pub fn item_ty(self) -> VdType {
        self.item_ty
    }

    pub fn expr_ty(self) -> VdType {
        self.expr_ty
    }
}
