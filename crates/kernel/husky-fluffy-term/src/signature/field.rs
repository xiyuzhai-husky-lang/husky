use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct FluffyFieldSignature {
    ty: FluffyTerm,
}

impl FluffyFieldSignature {
    pub fn ty(self) -> FluffyTerm {
        // match self {
        //     FieldEtherealSignature::PropsStruct(_) => todo!(),
        // }
        self.ty
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
        // ad hoc
        FluffyFieldSignature {
            ty: signature.ty().into(),
        }
    }
}

impl From<TypeMemoizedFieldEtherealSignature> for FluffyFieldSignature {
    fn from(signature: TypeMemoizedFieldEtherealSignature) -> Self {
        Self {
            // ad hoc
            ty: signature.return_ty().into(),
        }
    }
}
