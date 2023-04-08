use husky_signature::TraitForTypeImplBlockSignature;
use smallvec::SmallVec;

use super::*;

#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub(crate) struct TemplateParameters {
    #[return_ref]
    pub(super) data: SmallVec<[TemplateParameter; 2]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub(crate) struct TemplateParameter {
    symbol: TermSymbol,
    /// variable turned from the symbol
    variable: TermPlaceholder,
}

impl TemplateParameter {
    pub(crate) fn symbol(&self) -> TermSymbol {
        self.symbol
    }

    pub(crate) fn variable(&self) -> TermPlaceholder {
        self.variable
    }
}

impl TemplateParameters {
    fn new(
        db: &dyn TermDb,
        implicit_parameters: &[ImplicitParameterSignature],
    ) -> TermResult<Self> {
        Ok(Self::new_inner(
            db,
            implicit_parameters
                .iter()
                .rev()
                .map(|implicit_parameter| {
                    Ok(TemplateParameter {
                        symbol: TermSymbol::from_raw_unchecked(db, implicit_parameter.symbol())?,
                        variable: todo!(),
                    })
                })
                .rev()
                .collect::<TermResult<SmallVec<_>>>()?,
        ))
    }
}

pub(crate) trait HasTemplateParameters: Copy {
    fn template_parameters<'a>(self, db: &'a dyn TermDb) -> TermResult<TemplateParameters>;
}

impl HasTemplateParameters for TraitForTypeImplBlockSignature {
    fn template_parameters<'a>(self, db: &'a dyn TermDb) -> TermResult<TemplateParameters> {
        TemplateParameters::new(db, self.implicit_parameters(db))
    }
}

impl HasTemplateParameters for TypePath {
    fn template_parameters<'a>(self, db: &'a dyn TermDb) -> TermResult<TemplateParameters> {
        ty_path_template_parameters(db, self)
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn ty_path_template_parameters(
    db: &dyn TermDb,
    path: TypePath,
) -> TermResult<TemplateParameters> {
    TemplateParameters::new(db, path.signature(db)?.implicit_parameters(db))
}
