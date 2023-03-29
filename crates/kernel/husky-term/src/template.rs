use crate::*;
use husky_signature::ImplicitParameterSignature;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TemplateParameters {
    data: Vec<TemplateParameter>,
}

impl std::ops::Deref for TemplateParameters {
    type Target = [TemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TemplateParameter {
    symbol: TermSymbol,
    /// variable turned from the symbol
    variable: TermVariable,
}

impl TemplateParameters {
    fn new(
        db: &dyn TermDb,
        implicit_parameters: &[ImplicitParameterSignature],
    ) -> TermResult<Self> {
        Ok(Self {
            data: implicit_parameters
                .iter()
                .map(|implicit_parameter| {
                    Ok(TemplateParameter {
                        symbol: TermSymbol::from_raw_unchecked(db, implicit_parameter.symbol())?,
                        variable: todo!(),
                    })
                })
                .collect::<TermResult<Vec<_>>>()?,
        })
    }
}
