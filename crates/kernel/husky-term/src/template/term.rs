use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TemplateTerm(Term);

impl TemplateTerm {
    pub(super) fn new(
        db: &dyn TermDb,
        term: Term,
        template_parameters: &[TemplateParameter],
    ) -> Self {
        Self(term.to_template(db, template_parameters))
    }

    pub(super) fn term(self) -> Term {
        self.0
    }
}

impl Term {
    fn to_template(self, db: &dyn TermDb, parameters: &[TemplateParameter]) -> Self {
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
            Term::Literal(_) | Term::Variable(_) | Term::EntityPath(_) => unreachable!(),
            Term::Symbol(_) => todo!(),
            Term::Category(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }
}

impl TemplateTerm {
    #[inline(always)]
    pub(super) fn self_ty(
        db: &dyn TermDb,
        path: TypePath,
        template_parameters_data: &[TemplateParameter],
    ) -> Self {
        let mut ty: Term = TermEntityPath::TypeOntology(path).into();
        for template_parameter in template_parameters_data {
            ty = ty.apply_unchecked(db, template_parameter.variable(), 0)
        }
        TemplateTerm(ty)
    }
}
