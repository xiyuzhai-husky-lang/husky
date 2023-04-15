mod dyn_trai;
mod error;
mod trai_for_ty;
mod ty;

pub use self::error::*;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodType {
    indirections: SmallVec<[MethodIndirection; 2]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodIndirection {}

impl FluffyTerm {
    pub fn method_ty(self) -> FluffyMethodTypeResult<MethodType> {
        todo!()
    }
}
