use super::*;
use husky_print_utils::p;
use husky_raw_ty::ty_path_field_raw_ty;
use salsa::DebugWithDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RegularFieldCard {
    visibility: Visibility,
    modifier: FieldModifier,
    ty: EtherealTerm,
}

impl RegularFieldCard {
    fn leashed(self) -> Self {
        Self {
            visibility: self.visibility,
            modifier: FieldModifier::Leashed,
            ty: self.ty,
        }
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn modifier(&self) -> FieldModifier {
        self.modifier
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}

pub trait HasRegularFieldCard: Copy {
    fn regular_field_card(
        self,
        db: &dyn EtherealTypeDb,
        ident: Ident,
    ) -> TermResult<Option<RegularFieldCard>>;
}

impl HasRegularFieldCard for EtherealTerm {
    fn regular_field_card(
        self,
        db: &dyn EtherealTypeDb,
        ident: Ident,
    ) -> TermResult<Option<RegularFieldCard>> {
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(_) => todo!(),
            EtherealTerm::Placeholder(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => ty_ontology_path_field_ty(db, path, ident),
                TermEntityPath::TypeConstructor(_) => {
                    p!(self.debug(db), ident.debug(db));
                    todo!()
                }
            },
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => term_application_field_ty(db, term, ident),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

fn ty_ontology_path_field_ty(
    db: &dyn EtherealTypeDb,
    path: TypePath,
    ident: Ident,
) -> TermResult<Option<RegularFieldCard>> {
    let Some(field_raw_ty) = ty_path_field_raw_ty(db, path, ident)? else {
        return Ok(None)
    };
    Ok(Some(RegularFieldCard {
        visibility: todo!(),
        modifier: todo!(),
        ty: EtherealTerm::from_raw(
            db,
            field_raw_ty,
            TermTypeExpectation::FinalDestinationEqsSort,
        )?,
    }))
}

#[salsa::tracked(jar = EtherealTypeJar)]
pub(crate) fn term_application_field_ty(
    db: &dyn EtherealTypeDb,
    term: EtherealTermApplication,
    ident: Ident,
) -> TermResult<Option<RegularFieldCard>> {
    let expansion = term.application_expansion(db);
    let TermFunctionReduced::TypeOntology(path) = expansion.function() else {
        todo!("err")
    };
    match path.refine(db) {
        Right(PreludeTypePath::Borrow(path)) => match path {
            PreludeBorrowTypePath::Ref => todo!(),
            PreludeBorrowTypePath::RefMut => todo!(),
            PreludeBorrowTypePath::Leash => {
                let arguments = expansion.arguments(db);
                if arguments.len() != 1 {
                    todo!()
                }
                arguments[0]
                    .regular_field_card(db, ident)
                    .map(|opt| opt.map(|ty| ty.leashed()))
            }
        },
        _ => todo!(),
    }
}
