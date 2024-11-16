use super::*;
use either::*;
use visored_signature::signature::{
    attach::VdPowerSignature, binary_opr::base::VdBaseBinaryOprSignature,
    prefix_opr::VdBasePrefixOprSignature, separator::base::VdBaseSeparatorSignature,
};
use visored_term::instantiation::VdInstantiation;

pub mod menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdMirFunc {
    NormalBasePrefixOpr(VdBasePrefixOprSignature),
    NormalBaseSeparator(VdBaseSeparatorSignature),
    NormalBaseBinaryOpr(VdBaseBinaryOprSignature),
    Power(VdPowerSignature),
    InSet,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdMirFuncKey {
    NormalBasePrefixOpr(VdInstantiation),
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
            VdMirFunc::NormalBasePrefixOpr(signature) => {
                Left(VdMirFuncKey::NormalBasePrefixOpr(signature.instantiation()))
            }
            VdMirFunc::Power(signature) => Left(VdMirFuncKey::Power(signature.instantiation())),
            VdMirFunc::InSet => Left(VdMirFuncKey::InSet),
        }
    }
}
