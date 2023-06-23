use super::*;
use husky_ethereal_signature::{
    ExplicitParameterEtherealSignatureTemplate, ExplicitParameterKindTemplate,
};
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRitchieParameterContractedType {
    kind: FluffyExplicitParameterKind,
    contract: Contract,
    ty: FluffyTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyExplicitParameterKind {
    Regular,
    Keyed { ident: Ident },
}

impl From<TermRitchieParameterContractedType> for FluffyTermRitchieParameterContractedType {
    fn from(contracted_ty: TermRitchieParameterContractedType) -> Self {
        Self {
            contract: contracted_ty.contract(),
            ty: contracted_ty.ty().into(),
            kind: FluffyExplicitParameterKind::Regular,
        }
    }
}

impl Instantiator {
    pub(crate) fn instantiate_ritchie_parameter(
        &self,
        engine: &mut impl FluffyTermEngine,
        explicit_parameter: &ExplicitParameterEtherealSignatureTemplate,
    ) -> FluffyTermRitchieParameterContractedType {
        FluffyTermRitchieParameterContractedType {
            contract: explicit_parameter.contract(),
            ty: self.instantiate_term(engine, explicit_parameter.ty()),
            kind: self.instantiate_explicit_parameter_kind(explicit_parameter.kind()),
        }
    }

    #[inline(always)]
    fn instantiate_explicit_parameter_kind(
        &self,
        kind: ExplicitParameterKindTemplate,
    ) -> FluffyExplicitParameterKind {
        match kind {
            ExplicitParameterKindTemplate::Regular => FluffyExplicitParameterKind::Regular,
            ExplicitParameterKindTemplate::Keyed { ident } => {
                FluffyExplicitParameterKind::Keyed { ident }
            }
        }
    }
}

impl FluffyTermRitchieParameterContractedType {
    #[inline(always)]
    pub fn new(contract: Contract, ty: FluffyTerm) -> Self {
        Self {
            kind: FluffyExplicitParameterKind::Regular,
            contract,
            ty,
        }
    }

    pub fn kind(&self) -> FluffyExplicitParameterKind {
        self.kind
    }

    pub fn contract(self) -> Contract {
        self.contract
    }

    pub fn ty(self) -> FluffyTerm {
        self.ty
    }

    pub(crate) fn ty_mut(&mut self) -> &mut FluffyTerm {
        &mut self.ty
    }
}

pub struct FluffyTermRitchieVariadics {}

impl FluffyTerm {
    pub(crate) fn new_richie(
        db: &dyn FluffyTermDb,
        fluffy_terms: &mut FluffyTerms,
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: Vec<FluffyTermRitchieParameterContractedType>,
        return_ty: FluffyTerm,
    ) -> Self {
        let mut solid_flag = false;
        let mut hollow_flag = false;
        for parameter_contracted_ty in &parameter_contracted_tys {
            match parameter_contracted_ty.ty().nested() {
                NestedFluffyTerm::Ethereal(_) => (),
                NestedFluffyTerm::Solid(_) => solid_flag = true,
                NestedFluffyTerm::Hollow(_) => hollow_flag = true,
            }
        }
        match return_ty.nested() {
            NestedFluffyTerm::Ethereal(_) => (),
            NestedFluffyTerm::Solid(_) => solid_flag = true,
            NestedFluffyTerm::Hollow(_) => hollow_flag = true,
        }
        if hollow_flag {
            fluffy_terms
                .hollow_terms_mut()
                .alloc_new(HollowTermData::Ritchie {
                    ritchie_kind,
                    parameter_contracted_tys,
                    return_ty,
                })
                .into()
        } else if solid_flag {
            todo!()
        } else {
            todo!()
        }
    }
}
