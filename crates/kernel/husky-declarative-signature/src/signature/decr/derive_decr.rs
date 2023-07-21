use super::*;
use husky_syn_decr::{Decr, DeriveDecr, HasDecrs};
use vec_like::{SmallVecPairMap, SmallVecSet, VecMapGetEntry};

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveDecrDeclarativeSignatureTemplate {
    pub trai_term: DeclarativeTerm,
}

pub trait HasDeriveDecrDeclarativeSignatureTemplates: Copy {
    fn derive_decr_declarative_signature_templates_map(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &[(
            TraitPath,
            DeclarativeSignatureResult<SmallVecSet<DeriveDecrDeclarativeSignatureTemplate, 1>>,
        )],
    >;

    fn derive_decr_declarative_signature_templates(
        self,
        db: &dyn DeclarativeSignatureDb,
        trai_path: TraitPath,
    ) -> DeclarativeSignatureResult<Option<&[DeriveDecrDeclarativeSignatureTemplate]>> {
        match self
            .derive_decr_declarative_signature_templates_map(db)?
            .get_entry(trai_path)
        {
            Some((_, templates)) => Ok(Some(templates.as_ref()?)),
            None => Ok(None),
        }
    }
}

impl HasDeriveDecrDeclarativeSignatureTemplates for TypePath {
    fn derive_decr_declarative_signature_templates_map(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &[(
            TraitPath,
            DeclarativeSignatureResult<SmallVecSet<DeriveDecrDeclarativeSignatureTemplate, 1>>,
        )],
    > {
        Ok(ty_path_derive_decr_declarative_signature_templates_map(db, self).as_ref()?)
    }
}

// todo: change to ordered map and set
#[salsa::tracked(jar = DeclarativeSignatureJar, return_ref)]
fn ty_path_derive_decr_declarative_signature_templates_map(
    db: &dyn DeclarativeSignatureDb,
    ty_path: TypePath,
) -> DeclarativeSignatureResult<
    SmallVecPairMap<
        TraitPath,
        DeclarativeSignatureResult<SmallVecSet<DeriveDecrDeclarativeSignatureTemplate, 1>>,
        8,
    >,
> {
    let mut map: SmallVecPairMap<
        TraitPath,
        DeclarativeSignatureResult<SmallVecSet<DeriveDecrDeclarativeSignatureTemplate, 1>>,
        8,
    > = Default::default();
    for decr in ty_path.decrs(db)? {
        match decr {
            Decr::Derive(derive_decr) => {
                let expr_region = derive_decr.expr_region(db);
                let declarative_term_region = declarative_term_region(db, expr_region);
                for trai_expr in derive_decr.trai_exprs(db) {
                    let trai_term = declarative_term_region.expr_term(trai_expr.expr())?;
                    let template = DeriveDecrDeclarativeSignatureTemplate::new(db, trai_term);
                    // trai_term.a
                    let trai_path = match trai_term {
                        DeclarativeTerm::Literal(_) => todo!(),
                        DeclarativeTerm::Symbol(_) => todo!(),
                        DeclarativeTerm::Variable(_) => todo!(),
                        DeclarativeTerm::EntityPath(path) => match path {
                            DeclarativeTermEntityPath::Fugitive(_) => todo!(),
                            DeclarativeTermEntityPath::Trait(trai_path) => trai_path,
                            DeclarativeTermEntityPath::Type(_) => todo!(),
                            DeclarativeTermEntityPath::TypeVariant(_) => todo!(),
                        },
                        DeclarativeTerm::Category(_) => todo!(),
                        DeclarativeTerm::Universe(_) => todo!(),
                        DeclarativeTerm::Curry(_) => todo!(),
                        DeclarativeTerm::Ritchie(_) => todo!(),
                        DeclarativeTerm::Abstraction(_) => todo!(),
                        DeclarativeTerm::ExplicitApplication(_) => todo!(),
                        DeclarativeTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
                        DeclarativeTerm::Subentity(_) => todo!(),
                        DeclarativeTerm::AsTraitSubentity(_) => todo!(),
                        DeclarativeTerm::TraitConstraint(_) => todo!(),
                        DeclarativeTerm::LeashOrBitNot(_) => todo!(),
                        DeclarativeTerm::Wrapper(_) => todo!(),
                        DeclarativeTerm::List(_) => todo!(),
                    };
                    map.update_value_or_insert(
                        trai_path,
                        |_| todo!(),
                        Ok(SmallVecSet::new_one_elem_set(template)),
                    )
                }
            }
        }
    }
    Ok(map)
}
