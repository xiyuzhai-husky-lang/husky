use crate::*;
use husky_declarative_ty::principal_item_path::{
    fugitive_path_declarative_ty, trai_path_declarative_ty,
    ty_instance_constructor_path_declarative_ty, ty_ontology_path_declarative_ty,
    ty_variant::ty_variant_path_declarative_ty,
};
use husky_vfs::Toolchain;

pub trait HasType: Copy {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm>;
}

pub trait HasTypeGivenToolchain: Copy {
    fn ty(self, db: &::salsa::Db, toolchain: Toolchain) -> EtherealTermResult<EtherealTerm>;
}

pub trait HasTypeGivenDisambiguation: Copy {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm>;
}

impl HasTypeGivenDisambiguation for PrincipalEntityPath {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match self {
            PrincipalEntityPath::Module(path) => Ok(db
                .ethereal_term_menu(path.toolchain(db))
                .module_ty_ontology()),
            PrincipalEntityPath::MajorItem(path) => path.ty(db, disambiguation),
            PrincipalEntityPath::TypeVariant(path) => path.ty(db),
        }
    }
}

impl HasTypeGivenDisambiguation for MajorItemPath {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match self {
            MajorItemPath::Type(path) => path.ty(db, disambiguation),
            MajorItemPath::Trait(path) => path.ty(db),
            MajorItemPath::Fugitive(path) => path.ty(db),
        }
    }
}

impl HasType for TraitPath {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, trai_path_declarative_ty(db, self)?)
    }
}

impl HasType for FugitivePath {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, fugitive_path_declarative_ty(db, self)?)
    }
}

impl HasTypeGivenDisambiguation for TypePath {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match disambiguation {
            TypePathDisambiguation::OntologyConstructor => {
                EtherealTerm::ty_from_declarative(db, ty_ontology_path_declarative_ty(db, self)?)
            }
            TypePathDisambiguation::InstanceConstructor => EtherealTerm::ty_from_declarative(
                db,
                ty_instance_constructor_path_declarative_ty(db, self)?,
            ),
        }
    }
}

impl HasType for TypeVariantPath {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, ty_variant_path_declarative_ty(db, self)?)
    }
}

impl HasTypeGivenToolchain for EtherealTerm {
    fn ty(self, _db: &::salsa::Db, _toolchain: Toolchain) -> EtherealTermResult<EtherealTerm> {
        todo!()
        // self.ty_unchecked(db)?.checked(db)
    }
}

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawType {
    Prelude(PreludeTypePath),
    Declarative(DeclarativeTerm),
}

impl EtherealTerm {
    pub fn ty_unchecked(
        self,
        db: &::salsa::Db,
    ) -> EtherealTermResult<Either<EtherealTerm, PreludeTypePath>> {
        Ok(match self.raw_ty(db)? {
            RawType::Declarative(declarative_ty) => Left(EtherealTerm::from_declarative(
                db,
                declarative_ty,
                TermTypeExpectation::FinalDestinationEqsSort,
            )?),
            RawType::Prelude(prelude_ty_path) => Right(prelude_ty_path),
        })
    }

    pub fn raw_ty(self, db: &::salsa::Db) -> EtherealTermResult<RawType> {
        Ok(match self {
            EtherealTerm::Literal(slf) => RawType::Prelude(slf.ty()),
            EtherealTerm::Symbol(slf) => RawType::Declarative(slf.ty(db).into_declarative(db)),
            EtherealTerm::Rune(slf) => RawType::Declarative(slf.ty(db).into_declarative(db)),
            EtherealTerm::EntityPath(slf) => match slf {
                ItemPathTerm::Fugitive(path) => todo!(),
                ItemPathTerm::Trait(path) => {
                    RawType::Declarative(trai_path_declarative_ty(db, path)?)
                }
                ItemPathTerm::TypeOntology(path) => {
                    RawType::Declarative(ty_ontology_path_declarative_ty(db, path)?)
                }
                ItemPathTerm::TypeInstance(path) => {
                    RawType::Declarative(ty_instance_constructor_path_declarative_ty(db, path)?)
                }
                ItemPathTerm::TypeVariant(path) => {
                    RawType::Declarative(ty_variant_path_declarative_ty(db, path)?)
                }
            },
            EtherealTerm::Category(slf) => RawType::Declarative(slf.ty()?.into()),
            EtherealTerm::Universe(_) => RawType::Prelude(PreludeTypePath::UNIVERSE),
            EtherealTerm::Curry(slf) => slf.raw_ty(db),
            EtherealTerm::Ritchie(_) => {
                DeclarativeTerm::Category(CategoryTerm::new(1.into())).into()
            }
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => RawType::Declarative(term.declarative_ty(db)?),
            EtherealTerm::TypeAsTraitItem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        })
    }
}

impl CurryEtherealTerm {
    fn raw_ty(self, db: &salsa::Db) -> RawType {
        let Ok(RawType::Declarative(DeclarativeTerm::Category(parameter_ty_cat))) =
            self.parameter_ty(db).raw_ty(db)
        else {
            unreachable!()
        };
        let Ok(RawType::Declarative(DeclarativeTerm::Category(return_ty_cat))) =
            self.return_ty(db).raw_ty(db)
        else {
            unreachable!()
        };
        RawType::Declarative(parameter_ty_cat.max(return_ty_cat).into())
    }
}
