pub mod ritchie;

use husky_ethereal_signature::HasEtherealSignatureTemplate;
use husky_ethereal_term::{
    EtherealTerm, EtherealTermApplication, EtherealTermRitchie, EtherealTermSymbol,
    EtherealTermSymbolIndexInner, TermFunctionReduced,
};
use husky_term_prelude::TermEntityPath;

use crate::*;

/// this is much simpler than that in Term, right?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = HirTypeDb)]
pub enum HirType {
    PathLeading(HirTypePathLeading),
    Symbol(HirTypeSymbol),
    TypeAssociatedType(HirTypeTypeAssociatedType),
    TraitAssociatedType(HirTypeTraitAssociatedType),
    Ritchie(),
}

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
}

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTypeTypeAssociatedType {}

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTypeTraitAssociatedType {}

impl HirType {
    pub fn from_ethereal(term: EtherealTerm, db: &dyn HirTypeDb) -> Self {
        match term {
            EtherealTerm::Symbol(symbol) => HirTypeSymbol::from_ethereal(symbol, db).into(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(ty_path) => {
                    HirTypePathLeading::new(db, ty_path, smallvec![]).into()
                }
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Ritchie(term_ritchie) => {
                hir_ty_from_ethereal_term_ritchie(db, term_ritchie)
            }
            EtherealTerm::Application(term_application) => {
                hir_ty_from_ethereal_term_application(db, term_application)
            }
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            _ => unreachable!("it should be guaranteed that the term is a valid HirType"),
        }
    }

    pub fn prelude_ty_path(self, db: &dyn HirTypeDb) -> Option<PreludeTypePath> {
        match self {
            HirType::PathLeading(hir_ty) => hir_ty.ty_path(db).prelude_ty_path(db),
            _ => None,
        }
    }

    pub fn is_equal_to_unit_obviously(self, db: &dyn HirTypeDb) -> bool {
        match self.prelude_ty_path(db) {
            Some(PreludeTypePath::UNIT) => true,
            _ => false,
        }
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_ty_from_ethereal_term_ritchie(
    db: &dyn HirTypeDb,
    term_ritchie: EtherealTermRitchie,
) -> HirType {
    HirType::Ritchie()
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_ty_from_ethereal_term_application(
    db: &dyn HirTypeDb,
    term_application: EtherealTermApplication,
) -> HirType {
    let application_expansion = term_application.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => {
            let ty_ethereal_signature_template =
                ty_path.ethereal_signature_template(db).expect("ok");
            let template_parameters = ty_ethereal_signature_template.template_parameters(db);
            // filter out phantoms
            let template_arguments = std::iter::zip(
                template_parameters.iter(),
                application_expansion.arguments(db).iter().copied(),
            )
            .filter_map(|(param, arg)| {
                todo!("incorrect; redo this");
                let symbol = param.symbol();
                match symbol.index(db).inner() {
                    EtherealTermSymbolIndexInner::ExplicitLifetime {
                        attrs,
                        variance,
                        disambiguator,
                    } => todo!(),
                    EtherealTermSymbolIndexInner::ExplicitPlace {
                        attrs,
                        variance,
                        disambiguator,
                    } => Some(
                        HirPlaceSymbol {
                            attrs: HirSymbolAttrs::from_ethereal(attrs)?,
                            variance,
                            disambiguator,
                        }
                        .into(),
                    ),
                    EtherealTermSymbolIndexInner::Type { attrs, .. } => (!attrs.phantom())
                        .then(|| HirTemplateArgument::Type(HirType::from_ethereal(arg, db))),
                    EtherealTermSymbolIndexInner::Prop { .. } => None,
                    EtherealTermSymbolIndexInner::ConstPathLeading {
                        ty_path,
                        attrs,
                        disambiguator,
                    } => {
                        let ty = HirType::from_ethereal(symbol.ty(db), db);
                        Some(HirTemplateArgument::Constant(
                            HirConstSymbol::new(
                                db,
                                ty,
                                HirConstSymbolIndex::PathLeading {
                                    attrs: HirSymbolAttrs::from_ethereal(attrs)?,
                                    ty_path,
                                    disambiguator,
                                },
                            )
                            .into(),
                        ))
                    }
                    EtherealTermSymbolIndexInner::ConstOther {
                        attrs,
                        disambiguator,
                    } => {
                        let ty = HirType::from_ethereal(symbol.ty(db), db);
                        Some(HirTemplateArgument::Constant(
                            HirConstSymbol::new(
                                db,
                                ty,
                                HirConstSymbolIndex::Other {
                                    attrs: HirSymbolAttrs::from_ethereal(attrs)?,
                                    disambiguator,
                                },
                            )
                            .into(),
                        ))
                    }
                    EtherealTermSymbolIndexInner::EphemPathLeading { .. }
                    | EtherealTermSymbolIndexInner::EphemOther { .. }
                    | EtherealTermSymbolIndexInner::SelfType
                    | EtherealTermSymbolIndexInner::SelfValue
                    | EtherealTermSymbolIndexInner::SelfLifetime
                    | EtherealTermSymbolIndexInner::SelfPlace => unreachable!(),
                }
            })
            .collect();
            HirTypePathLeading::new(db, ty_path, template_arguments).into()
        }
        TermFunctionReduced::Trait(_) => todo!(),
        TermFunctionReduced::Other(_) => todo!(),
    }
}

/// excluding symbols
#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTypeLiteral {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArgumentLiterals,
}
