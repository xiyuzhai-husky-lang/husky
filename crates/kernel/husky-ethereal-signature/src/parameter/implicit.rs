use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb)]
pub struct ImplicitParameterEtherealSignatures {
    data: SmallVec<[ImplicitParameterEtherealSignature; 2]>,
}

impl ImplicitParameterEtherealSignatures {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        implicit_paramters: &ImplicitParameterDeclarativeSignatures,
    ) -> EtherealSignatureResult<ImplicitParameterEtherealSignatures> {
        Ok(ImplicitParameterEtherealSignatures {
            data: implicit_paramters
                .data()
                .iter()
                .map(|implicit_parameter| {
                    ImplicitParameterEtherealSignature::from_declarative(db, implicit_parameter)
                })
                .collect::<EtherealSignatureResult<_>>()?,
        })
    }

    #[inline(always)]
    pub fn data(&self) -> &[ImplicitParameterEtherealSignature] {
        &self.data
    }
}

impl std::ops::Deref for ImplicitParameterEtherealSignatures {
    type Target = [ImplicitParameterEtherealSignature];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl EtherealInstantiateRef for ImplicitParameterEtherealSignatures {
    type Target = Self;

    fn instantiate(
        &self,
        db: &dyn EtherealSignatureDb,
        instantiator: &EtherealInstantiator,
    ) -> Self::Target {
        ImplicitParameterEtherealSignatures {
            data: self
                .data
                .iter()
                .filter_map(|param| param.instantiate(db, instantiator))
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterEtherealSignature {
    annotated_variance: Option<Variance>,
    symbol: EtherealTermSymbol,
    traits: Vec<EtherealTerm>,
}

impl ImplicitParameterEtherealSignature {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        implicit_paramter: &ImplicitParameterDeclarativeSignature,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            annotated_variance: implicit_paramter.annotated_variance(),
            symbol: EtherealTermSymbol::from_declarative(db, implicit_paramter.symbol())?,
            traits: implicit_paramter.traits().iter().map(|_| todo!()).collect(),
        })
    }

    pub fn symbol(&self) -> EtherealTermSymbol {
        self.symbol
    }

    pub fn traits(&self) -> &[EtherealTerm] {
        self.traits.as_ref()
    }
}

impl EtherealInstantiateRef for ImplicitParameterEtherealSignature {
    type Target = Option<Self>;

    fn instantiate(
        &self,
        db: &dyn EtherealSignatureDb,
        instantiator: &EtherealInstantiator,
    ) -> Self::Target {
        if instantiator.is_symbol_resolved(self.symbol) {
            return None;
        }
        Some(ImplicitParameterEtherealSignature {
            annotated_variance: self.annotated_variance,
            // ad hoc
            // make new_inner private
            symbol: EtherealTermSymbol::new_inner(
                db,
                self.symbol.ty(db).instantiate(db, instantiator),
                self.symbol.idx(db),
            ),
            traits: self.traits.iter().map(|_| todo!()).collect(),
        })
    }
}
