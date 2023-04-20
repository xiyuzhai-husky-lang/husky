use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TemplateTerm(EtherealTerm);

impl TemplateTerm {
    pub(super) fn new(
        db: &dyn EtherealTermDb,
        term: EtherealTerm,
        template_parameters: &[TemplateParameter],
    ) -> Self {
        Self(term.to_template(db, template_parameters))
    }

    pub(super) fn term(self) -> EtherealTerm {
        self.0
    }
}

impl EtherealTerm {
    fn to_template(self, db: &dyn EtherealTermDb, parameters: &[TemplateParameter]) -> Self {
        let Some(symbols) = self.symbols(db) else {
            return self
        };
        let symbols = symbols.data(db);
        if !symbols.iter().copied().any(|symbol| {
            parameters
                .iter()
                .any(|parameter| symbol == parameter.symbol())
        }) {
            return self;
        }
        match self {
            EtherealTerm::Literal(_) | EtherealTerm::Variable(_) | EtherealTerm::EntityPath(_) => {
                unreachable!()
            }
            EtherealTerm::Symbol(_) => todo!(),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

impl TemplateTerm {
    #[inline(always)]
    pub(super) fn self_ty(
        db: &dyn EtherealTermDb,
        path: TypePath,
        template_parameters_data: &[TemplateParameter],
    ) -> Self {
        let mut ty: EtherealTerm = TermEntityPath::TypeOntology(path).into();
        for template_parameter in template_parameters_data {
            ty = ty.apply_unchecked(db, template_parameter.variable(), 0)
        }
        TemplateTerm(ty)
    }
}
