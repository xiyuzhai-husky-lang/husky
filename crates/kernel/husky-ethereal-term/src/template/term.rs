use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct TemplateTerm(EtherealTerm);

impl TemplateTerm {
    pub(super) fn new(
        db: &dyn EtherealTermDb,
        term: EtherealTerm,
        template_parameters: &[TemplateParameter],
    ) -> Self {
        Self(term)
    }

    pub(super) fn term(self) -> EtherealTerm {
        self.0
    }
}

impl TemplateTerm {
    #[inline(always)]
    pub(super) fn self_ty(
        db: &dyn EtherealTermDb,
        path: TypePath,
        template_parameters: &[TemplateParameter],
    ) -> Self {
        let mut ty: EtherealTerm = TermEntityPath::TypeOntology(path).into();
        for template_parameter in template_parameters {
            ty = ty.apply_unchecked(db, template_parameter.symbol(), 0)
        }
        TemplateTerm(ty)
    }
}
