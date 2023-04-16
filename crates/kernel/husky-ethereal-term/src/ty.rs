use crate::*;
use husky_declarative_ty::*;
use husky_ty_expectation::TypePathDisambiguation;
use husky_vfs::Toolchain;

pub trait HasType: Copy {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm>;
}

pub trait HasTypeGivenToolchain: Copy {
    fn ty(self, db: &dyn EtherealTermDb, toolchain: Toolchain) -> TermResult<EtherealTerm>;
}

pub trait HasTypeGivenDisambiguation: Copy {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm>;
}

impl HasTypeGivenDisambiguation for EntityPath {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        match self {
            EntityPath::Module(path) => Ok(db.term_menu(path.toolchain(db)).module_ty_ontology()),
            EntityPath::ModuleItem(path) => path.ty(db, disambiguation),
            EntityPath::AssociatedItem(path) => path.ty(db),
            EntityPath::TypeVariant(path) => path.ty(db),
        }
    }
}

impl HasTypeGivenDisambiguation for ModuleItemPath {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        match self {
            ModuleItemPath::Type(path) => path.ty(db, disambiguation),
            ModuleItemPath::Trait(path) => path.ty(db),
            ModuleItemPath::Form(path) => path.ty(db),
        }
    }
}

impl HasType for TraitPath {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm> {
        EtherealTerm::ty_from_raw(db, trai_path_raw_ty(db, self)?)
    }
}

impl HasType for FormPath {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm> {
        EtherealTerm::ty_from_raw(db, form_path_raw_ty(db, self)?)
    }
}

impl HasTypeGivenDisambiguation for TypePath {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        match disambiguation {
            TypePathDisambiguation::Ontology => {
                EtherealTerm::ty_from_raw(db, ty_ontology_path_raw_ty(db, self)?)
            }
            TypePathDisambiguation::Constructor => {
                EtherealTerm::ty_from_raw(db, ty_constructor_path_raw_ty(db, self)?)
            }
        }
    }
}

impl HasType for AssociatedItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm> {
        match self {
            AssociatedItemPath::TypeItem(path) => path.ty(db),
            AssociatedItemPath::TraitItem(path) => path.ty(db),
            AssociatedItemPath::TraitForTypeItem(path) => path.ty(db),
        }
    }
}

impl HasType for TypeItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm> {
        EtherealTerm::ty_from_raw_unchecked(db, self.raw_ty(db)?)
    }
}

impl HasType for TraitItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm> {
        todo!()
    }
}

impl HasType for TraitForTypeItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm> {
        todo!()
    }
}

impl HasType for TypeVariantPath {
    fn ty(self, db: &dyn EtherealTermDb) -> TermResult<EtherealTerm> {
        EtherealTerm::ty_from_raw(db, ty_variant_path_raw_ty(db, self)?)
    }
}

impl HasTypeGivenToolchain for EtherealTerm {
    fn ty(self, db: &dyn EtherealTermDb, toolchain: Toolchain) -> TermResult<EtherealTerm> {
        todo!()
        // self.ty_unchecked(db)?.checked(db)
    }
}

impl EtherealTerm {
    pub fn ty_unchecked(
        self,
        db: &dyn EtherealTermDb,
    ) -> TermResult<Either<EtherealTerm, PreludeTypePath>> {
        Ok(match self.raw_ty(db)? {
            Left(raw_ty) => Left(EtherealTerm::from_raw_unchecked(
                db,
                raw_ty,
                TermTypeExpectation::FinalDestinationEqsSort,
            )?),
            Right(prelude_ty_path) => Right(prelude_ty_path),
        })
    }

    pub fn raw_ty(
        self,
        db: &dyn EtherealTermDb,
    ) -> TermResult<Either<DeclarativeTerm, PreludeTypePath>> {
        Ok(match self {
            EtherealTerm::Literal(literal) => Right(literal.ty()),
            // term.raw_ty(db),
            EtherealTerm::Symbol(symbol) => todo!(),
            EtherealTerm::Placeholder(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(path) => Left(trai_path_raw_ty(db, path)?),
                TermEntityPath::TypeOntology(path) => Left(ty_ontology_path_raw_ty(db, path)?),
                TermEntityPath::TypeConstructor(path) => {
                    Left(ty_constructor_path_raw_ty(db, path)?)
                }
            },
            EtherealTerm::Category(cat) => Left(cat.ty()?.into()),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => Left(term.raw_ty(db)?),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        })
    }
}
