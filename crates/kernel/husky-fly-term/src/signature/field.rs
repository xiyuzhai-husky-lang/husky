use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FlyFieldSignature {
    PropsStruct {
        ty: FlyTerm,
    },
    Memoized {
        ty: FlyTerm,
        path: AssociatedItemPath,
        instantiation: FlyInstantiation,
    },
}

impl FlyFieldSignature {
    pub fn return_ty(&self) -> FlyTerm {
        match *self {
            FlyFieldSignature::PropsStruct { ty } => ty,
            FlyFieldSignature::Memoized { ty, .. } => ty,
        }
    }
}

impl MemberSignature for FlyFieldSignature {
    fn expr_ty(&self, self_value_final_place: FlyPlace) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}

impl From<PropsFieldEtherealSignature> for FlyFieldSignature {
    fn from(signature: PropsFieldEtherealSignature) -> Self {
        match signature {
            PropsFieldEtherealSignature::PropsStruct(signature) => FlyFieldSignature::PropsStruct {
                ty: signature.ty().into(),
            },
        }
    }
}

impl From<TypeMemoizedFieldEtherealSignature> for FlyFieldSignature {
    fn from(signature: TypeMemoizedFieldEtherealSignature) -> Self {
        FlyFieldSignature::Memoized {
            // ad hoc
            ty: signature.return_ty().into(),
            path: signature.path().into(),
            instantiation: FlyInstantiation::from_eth(
                FlyInstantiationEnvironment::MemoizedField,
                signature.instantiation(),
            ),
        }
    }
}
