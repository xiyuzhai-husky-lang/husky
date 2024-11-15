use super::*;
use either::*;
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_zfc_ty::instantiation::VdInstantiation;

pub mod menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdMirFunc {
    NormalBaseSeparator(VdBaseSeparatorSignature),
    InSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdMirFuncKey {
    NormalBaseSeparator(VdInstantiation),
    InSet,
}

impl VdMirFunc {
    pub fn key_or_expr(self, db: &::salsa::Db) -> Either<VdMirFuncKey, VdMirExprIdx> {
        match self {
            VdMirFunc::NormalBaseSeparator(signature) => Left(VdMirFuncKey::NormalBaseSeparator(
                signature.instantiation(db),
            )),
            VdMirFunc::InSet => Left(VdMirFuncKey::InSet),
        }
    }
}
