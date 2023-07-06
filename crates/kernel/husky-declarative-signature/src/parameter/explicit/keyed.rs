use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitKeyedParameterDeclarativeSignatureTemplate {
    key: Ident,
    contract: Contract,
    ty: DeclarativeTerm,
    default: Option<DeclarativeTerm>,
}

impl ExplicitKeyedParameterDeclarativeSignatureTemplate {
    pub(crate) fn new(
        key: Ident,
        contract: Contract,
        ty: DeclarativeTerm,
        default: Option<DeclarativeTerm>,
    ) -> Self {
        Self {
            key,
            contract,
            ty,
            default,
        }
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    pub fn default(&self) -> Option<DeclarativeTerm> {
        self.default
    }
}
