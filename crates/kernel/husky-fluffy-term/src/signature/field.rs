use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub enum FluffyFieldSignature {
    PropsStruct { ty: FluffyTerm },
    Memoized { ty: FluffyTerm },
}

impl FluffyFieldSignature {
    pub fn return_ty(self) -> FluffyTerm {
        // match self {
        //     FieldEtherealSignature::PropsStruct(_) => todo!(),
        // }
        // ad hoc
        match self {
            FluffyFieldSignature::PropsStruct { ty } => ty,
            FluffyFieldSignature::Memoized { ty } => ty,
        }
    }
}

impl MemberSignature for FluffyFieldSignature {
    fn expr_ty(
        &self,
        indirections: &[FluffyDynamicDispatchIndirection],
    ) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}

impl From<PropsFieldEtherealSignature> for FluffyFieldSignature {
    fn from(signature: PropsFieldEtherealSignature) -> Self {
        match signature {
            PropsFieldEtherealSignature::PropsStruct(signature) => {
                FluffyFieldSignature::PropsStruct {
                    ty: signature.ty().into(),
                }
            }
        }
    }
}

impl From<TypeMemoizedFieldEtherealSignature> for FluffyFieldSignature {
    fn from(signature: TypeMemoizedFieldEtherealSignature) -> Self {
        FluffyFieldSignature::Memoized {
            // ad hoc
            ty: signature.return_ty().into(),
        }
    }
}
