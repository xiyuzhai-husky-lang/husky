mod trai;
mod trai_for_ty;
mod ty;

pub use self::trai::*;
pub use self::trai_for_ty::*;
pub use self::ty::*;

use crate::*;
use husky_decl::{TypeItemDecl, TypeMethodFnDecl};
use husky_declarative_ty::ty_path_ty_method_raw_ty;
use husky_entity_tree::{AssociatedItemId, EntityTreeBundleResult};
use husky_signature::{SignatureResult, TypeMethodSignature};
use husky_word::IdentPairMap;

#[salsa::tracked(db = EtherealTypeDb, jar = EtherealTypeJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnCard {
    #[id]
    id: AssociatedItemId,
    method_ty: TermResult<EtherealTerm>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MethodTypeInfo {
    implicit_parameters: Vec<EtherealTermSymbol>,
    self_contracted_ty: TermRitchieParameterContractedType,
    nonself_parameter_contracted_tys: Vec<TermRitchieParameterContractedType>,
    return_ty: EtherealTerm,
    where_clause: (),
}

impl MethodTypeInfo {
    fn ty(&self, db: &dyn EtherealTypeDb) -> TermResult<EtherealTerm> {
        match self {
            MethodTypeInfo {
                implicit_parameters,
                self_contracted_ty,
                nonself_parameter_contracted_tys,
                return_ty,
                where_clause,
            } => {
                let mut parameter_tys = vec![];
                let mut method_ty: EtherealTerm = EtherealTermRitchie::new(
                    db,
                    TermRitchieKind::FnType,
                    parameter_tys,
                    *return_ty,
                )?
                .into();
                for implicit_signature in implicit_parameters.iter().copied() {
                    todo!()
                }
                Ok(method_ty)
                // method_ty.checked(db)
            }
        }
    }

    pub fn implicit_parameters(&self) -> &[EtherealTermSymbol] {
        self.implicit_parameters.as_ref()
    }

    pub fn self_contracted_ty(&self) -> TermRitchieParameterContractedType {
        self.self_contracted_ty
    }

    pub fn nonself_parameter_contracted_tys(&self) -> &[TermRitchieParameterContractedType] {
        self.nonself_parameter_contracted_tys.as_ref()
    }

    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }

    pub fn where_clause(&self) -> () {
        self.where_clause
    }
}
