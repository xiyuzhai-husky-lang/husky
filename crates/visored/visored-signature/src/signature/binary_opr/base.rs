use super::*;
use visored_entity_path::path::{trai_item::VdTraitItemPath, VdItemPath};
use visored_opr::opr::binary::VdBaseBinaryOpr;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VdBaseBinaryOprSignature {
    pub instantiation: VdInstantiation,
    // TODO: replace this with something more ethereal
    pub opr: VdBaseBinaryOpr,
    pub lopd_ty: VdType,
    pub ropd_ty: VdType,
    pub expr_ty: VdType,
}

impl From<VdBaseBinaryOprSignature> for VdSignature {
    fn from(signature: VdBaseBinaryOprSignature) -> Self {
        VdSignature::BinaryOpr(signature.into())
    }
}

impl VdBaseBinaryOprSignature {
    pub fn new(
        instantiation: VdInstantiation,
        lopd_ty: VdType,
        ropd_ty: VdType,
        expr_ty: VdType,
    ) -> Self {
        let VdItemPath::TraitItem(path) = instantiation.path() else {
            unreachable!()
        };
        let opr = match path {
            VdTraitItemPath::GroupMul => todo!(),
            VdTraitItemPath::AbelianGroupAdd => todo!(),
            VdTraitItemPath::NatAdd => todo!(),
            VdTraitItemPath::NatMul => todo!(),
            VdTraitItemPath::RingAdd => todo!(),
            VdTraitItemPath::RingSub => VdBaseBinaryOpr::SUB,
            VdTraitItemPath::RingMul => todo!(),
            VdTraitItemPath::RingPower => todo!(),
            VdTraitItemPath::RingPos => todo!(),
            VdTraitItemPath::RingNeg => todo!(),
            VdTraitItemPath::Eq => todo!(),
            VdTraitItemPath::Ne => todo!(),
            VdTraitItemPath::Lt => todo!(),
            VdTraitItemPath::Gt => todo!(),
            VdTraitItemPath::Le => todo!(),
            VdTraitItemPath::Ge => todo!(),
            VdTraitItemPath::FieldDiv => VdBaseBinaryOpr::DIV,
            VdTraitItemPath::RealSqrt => todo!(),
        };
        Self {
            instantiation,
            opr,
            lopd_ty,
            ropd_ty,
            expr_ty,
        }
    }
}

impl VdBaseBinaryOprSignature {
    pub fn instantiation(self) -> VdInstantiation {
        self.instantiation
    }

    pub fn lopd_ty(self) -> VdType {
        self.lopd_ty
    }

    pub fn ropd_ty(self) -> VdType {
        self.ropd_ty
    }

    pub fn expr_ty(self) -> VdType {
        self.expr_ty
    }
}
