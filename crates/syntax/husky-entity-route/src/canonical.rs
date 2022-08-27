use crate::EntityRoutePtr;

// todo: mutable ref
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonicalTy {
    is_option: bool,
    qual: CanonicalQualifier,
    intrinsic_ty: EntityRoutePtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalQualifier {
    Intrinsic,
    EvalRef,
    TempRef,
    TempRefMut,
}

impl std::fmt::Display for CanonicalTyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanonicalTyKind::Intrinsic => "Intrinsic",
            CanonicalTyKind::Optional => "Optional",
            CanonicalTyKind::EvalRef => "EvalRef",
            CanonicalTyKind::OptionalEvalRef => "OptionalEvalRef",
            CanonicalTyKind::TempRefMut => todo!(),
        }
        .fmt(f)
    }
}

impl CanonicalTy {
    #[inline(always)]
    pub fn intrinsic_ty(self) -> EntityRoutePtr {
        self.intrinsic_ty
    }
    pub fn new(is_option: bool, qual: CanonicalQualifier, intrinsic_ty: EntityRoutePtr) -> Self {
        assert!(intrinsic_ty.is_intrinsic());
        Self {
            is_option,
            qual,
            intrinsic_ty,
        }
    }

    pub fn with_eval_ref(self) -> Self {
        Self {
            is_option: self.is_option,
            qual: match self.qual {
                CanonicalQualifier::Intrinsic | CanonicalQualifier::EvalRef => {
                    CanonicalQualifier::EvalRef
                }
                CanonicalQualifier::TempRef => todo!(),
                CanonicalQualifier::TempRefMut => todo!(),
            },
            intrinsic_ty: self.intrinsic_ty,
        }
    }

    pub fn is_option(&self) -> bool {
        self.is_option
    }

    pub fn kind(&self) -> CanonicalTyKind {
        match (self.is_option, self.qual) {
            (true, CanonicalQualifier::Intrinsic) => CanonicalTyKind::Optional,
            (true, CanonicalQualifier::EvalRef) => CanonicalTyKind::OptionalEvalRef,
            (true, CanonicalQualifier::TempRef) => todo!(),
            (true, CanonicalQualifier::TempRefMut) => todo!(),
            (false, CanonicalQualifier::Intrinsic) => CanonicalTyKind::Intrinsic,
            (false, CanonicalQualifier::EvalRef) => CanonicalTyKind::EvalRef,
            (false, CanonicalQualifier::TempRef) => todo!(),
            (false, CanonicalQualifier::TempRefMut) => CanonicalTyKind::TempRefMut,
        }
    }

    pub fn is_intrinsic_route_primitive(&self) -> bool {
        self.intrinsic_ty.is_primitive()
    }

    pub fn qual(self) -> CanonicalQualifier {
        self.qual
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalTyKind {
    Intrinsic,
    Optional,
    EvalRef,
    OptionalEvalRef,
    TempRefMut,
}
