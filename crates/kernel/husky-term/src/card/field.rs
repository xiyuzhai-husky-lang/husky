use super::*;
use husky_raw_ty::ty_path_field_raw_ty;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RegularFieldCard {
    visibility: Visibility,
    modifier: FieldModifier,
    ty: Term,
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

    pub fn ty(&self) -> Term {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldModifier {
    Pure,
    Mut,
    Const,
    Leashed,
}

impl Term {
    pub fn regular_field_card(
        self,
        db: &dyn TermDb,
        ident: Ident,
    ) -> TermResult<Option<RegularFieldCard>> {
        match self {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::Hole(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => ty_ontology_path_field_ty(db, path, ident),
                TermEntityPath::TypeConstructor(_) => {
                    p!(self.debug(db), ident.debug(db));
                    todo!()
                }
            },
            Term::Category(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(term) => term_application_field_ty(db, term, ident),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }
}

fn ty_ontology_path_field_ty(
    db: &dyn TermDb,
    path: TypePath,
    ident: Ident,
) -> TermResult<Option<RegularFieldCard>> {
    let Some(field_raw_ty) = ty_path_field_raw_ty(db, path, ident)? else {
        return Ok(None)
    };
    Ok(Some(RegularFieldCard {
        visibility: todo!(),
        modifier: todo!(),
        ty: Term::from_raw_unchecked(
            db,
            field_raw_ty,
            TermTypeExpectation::FinalDestinationEqsSort,
        )?,
    }))
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_application_field_ty(
    db: &dyn TermDb,
    term: TermApplication,
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
