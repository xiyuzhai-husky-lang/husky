use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub enum FluffyFieldSignature {
    PropsStruct {
        ty: FluffyTerm,
    },
    Memoized {
        ty: FluffyTerm,
        path: AssociatedItemPath,
        instantiation: FluffyInstantiation,
    },
}

impl FluffyFieldSignature {
    pub fn return_ty(&self) -> FluffyTerm {
        match *self {
            FluffyFieldSignature::PropsStruct { ty } => ty,
            FluffyFieldSignature::Memoized { ty, .. } => ty,
        }
    }
}

impl MemberSignature for FluffyFieldSignature {
    fn expr_ty(&self, self_value_final_place: FluffyPlace) -> FluffyTermResult<FluffyTerm> {
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
            path: signature.path().into(),
            instantiation: FluffyInstantiation::from_ethereal(
                FluffyInstantiationEnvironment::MemoizedField,
                signature.instantiation(),
            ),
        }
    }
}
