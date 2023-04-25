use husky_declarative_signature::TraitForTypeImplBlockDeclarativeSignatureTemplate;
use smallvec::SmallVec;

use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub(crate) struct TemplateParameters {
    #[return_ref]
    pub(super) data: SmallVec<[TemplateParameter; 2]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub(crate) struct TemplateParameter {
    symbol: EtherealTermSymbol,
    /// variable turned from the symbol
    variable: EtherealTermVariable,
}

impl TemplateParameter {
    pub(crate) fn symbol(&self) -> EtherealTermSymbol {
        self.symbol
    }

    pub(crate) fn variable(&self) -> EtherealTermVariable {
        self.variable
    }
}

impl TemplateParameters {
    fn new(
        db: &dyn EtherealTermDb,
        implicit_parameters: &[ImplicitParameterDeclarativeSignature],
    ) -> EtherealTermResult<Self> {
        Ok(Self::new_inner(
            db,
            implicit_parameters
                .iter()
                .rev()
                .map(|implicit_parameter| {
                    Ok(TemplateParameter {
                        symbol: EtherealTermSymbol::from_declarative(
                            db,
                            implicit_parameter.symbol(),
                        )?,
                        variable: todo!(),
                    })
                })
                .rev()
                .collect::<EtherealTermResult<SmallVec<_>>>()?,
        ))
    }
}

pub(crate) trait HasTemplateParameters: Copy {
    fn template_parameters<'a>(
        self,
        db: &'a dyn EtherealTermDb,
    ) -> EtherealTermResult<TemplateParameters>;
}

impl HasTemplateParameters for TraitForTypeImplBlockDeclarativeSignatureTemplate {
    fn template_parameters<'a>(
        self,
        db: &'a dyn EtherealTermDb,
    ) -> EtherealTermResult<TemplateParameters> {
        TemplateParameters::new(db, self.implicit_parameters(db))
    }
}

impl HasTemplateParameters for TypePath {
    fn template_parameters<'a>(
        self,
        db: &'a dyn EtherealTermDb,
    ) -> EtherealTermResult<TemplateParameters> {
        ty_path_template_parameters(db, self)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ty_path_template_parameters(
    db: &dyn EtherealTermDb,
    path: TypePath,
) -> EtherealTermResult<TemplateParameters> {
    TemplateParameters::new(
        db,
        path.declarative_signature_template(db)?
            .implicit_parameters(db),
    )
}
