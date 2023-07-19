use super::*;
use husky_decr::{Decr, DeriveDecr, HasDecrs};
use vec_like::SmallVecPairMap;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveDecrDeclarativeSignatureTemplate {
    pub trai: DeclarativeTerm,
}

// impl HasDeclarativeSignatureTemplate for DeriveDecr {
//     type DeclarativeSignatureTemplate = DeriveDecrDeclarativeSignatureTemplate;

//     fn declarative_signature_template(
//         self,
//         db: &dyn DeclarativeSignatureDb,
//     ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
//         derive_decr_declarative_signature_template(db, self)
//     }
// }

// #[salsa::tracked(jar = DeclarativeSignatureJar)]
// pub fn derive_decr_declarative_signature_template(
//     db: &dyn DeclarativeSignatureDb,
//     derive_decr: DeriveDecr,
// ) -> DeclarativeSignatureResult<DeriveDecrDeclarativeSignatureTemplate> {
//     let expr_region = derive_decr.expr_region(db);
//     let declarative_term_region = declarative_term_region(db, expr_region);
//     let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
//     let traits = derive_decr
//         .traits(db)
//         .iter()
//         .copied()
//         .map(|trai_expr| declarative_term_region.expr_term(trai_expr.expr()))
//         .collect::<DeclarativeTermResultBorrowed2<Vec<_>>>()?;
//     Ok(DeriveDecrDeclarativeSignatureTemplate::new(db, traits))
// }

pub trait HasDeriveDecrDeclarativeSignatureTemplates: Copy {
    fn derive_decr_declarative_signature_templates_map(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &[(
            TraitPath,
            SmallVec<[DeriveDecrDeclarativeSignatureTemplate; 1]>,
        )],
    >;

    fn derive_decr_declarative_signature_templates(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Option<&[DeriveDecrDeclarativeSignatureTemplate]>> {
        todo!()
    }
}

impl HasDeriveDecrDeclarativeSignatureTemplates for TypePath {
    fn derive_decr_declarative_signature_templates_map(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &[(
            TraitPath,
            SmallVec<[DeriveDecrDeclarativeSignatureTemplate; 1]>,
        )],
    > {
        Ok(ty_path_derive_decr_declarative_signature_templates_map(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar, return_ref)]
fn ty_path_derive_decr_declarative_signature_templates_map(
    db: &dyn DeclarativeSignatureDb,
    ty_path: TypePath,
) -> DeclarativeSignatureResult<
    SmallVecPairMap<TraitPath, SmallVec<[DeriveDecrDeclarativeSignatureTemplate; 1]>, 8>,
> {
    let mut map: SmallVecPairMap<
        TraitPath,
        SmallVec<[DeriveDecrDeclarativeSignatureTemplate; 1]>,
        8,
    > = Default::default();
    for decr in ty_path.decrs(db)? {
        match decr {
            Decr::Derive(derive_decr) => {
                let expr_region = derive_decr.expr_region(db);
                let declarative_term_region = declarative_term_region(db, expr_region);
                for trai_expr in derive_decr.trai_exprs(db) {
                    declarative_term_region.expr_term(trai_expr.expr());
                }
                todo!()
            }
        }
    }
    Ok(map)
}
