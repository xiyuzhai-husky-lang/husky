use super::*;
use either::*;
use visored_signature::signature::{
    attach::VdPowerSignature, binary_opr::base::VdBaseBinaryOprSignature,
    separator::base::VdBaseSeparatorSignature,
};
use visored_zfc_ty::instantiation::VdInstantiation;

pub mod menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdMirFunc {
    NormalBaseSeparator(VdBaseSeparatorSignature),
    NormalBaseBinaryOpr(VdBaseBinaryOprSignature),
    Power(VdPowerSignature),
    InSet,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdMirFuncKey {
    NormalBaseSeparator(VdInstantiation),
    NormalBaseBinaryOpr(VdInstantiation),
    InSet,
    Power(VdInstantiation),
}

impl VdMirFunc {
    pub fn key_or_expr(self, db: &::salsa::Db) -> Either<VdMirFuncKey, VdMirExprIdx> {
        match self {
            VdMirFunc::NormalBaseSeparator(signature) => {
                Left(VdMirFuncKey::NormalBaseSeparator(signature.instantiation()))
            }
            VdMirFunc::NormalBaseBinaryOpr(signature) => {
                Left(VdMirFuncKey::NormalBaseBinaryOpr(signature.instantiation()))
            }
            VdMirFunc::Power(signature) => Left(VdMirFuncKey::Power(signature.instantiation())),
            VdMirFunc::InSet => Left(VdMirFuncKey::InSet),
        }
    }
}
