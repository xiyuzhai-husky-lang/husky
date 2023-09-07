use super::*;
use husky_entity_syn_tree::HasDecrPaths;
use vec_like::{SmallVecPairMap, SmallVecSet, VecMapGetEntry};

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveDecrDeclarativeSignatureTemplate {
    pub shards: SmallVec<[DeriveDecrShardDeclarativeSignatureTemplate; 8]>,
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveDecrShardDeclarativeSignatureTemplate {
    pub trai_term: DeclarativeTerm,
}

impl DeriveDecrDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        decl: DeriveDecrSynDecl,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let shards = decl
            .trais(db)
            .iter()
            .map(|trai| {
                Ok(DeriveDecrShardDeclarativeSignatureTemplate::new(
                    db,
                    declarative_term_region.expr_term(trai.expr())?,
                ))
            })
            .collect::<DeclarativeTermResultBorrowed2<_>>()?;
        Ok(DeriveDecrDeclarativeSignatureTemplate::new(db, shards))
    }
}

// // todo: change to ordered map and set
//     for decr_path in ty_path.decr_paths(db) {
//         match decr_path.syn_decl(db)? {
//             DecrSynDecl::Derive(derive_decr) => {
//                 let syn_expr_region = derive_decr.syn_expr_region(db);
//                 let declarative_term_region = declarative_term_region(db, syn_expr_region);
//                 for trai in derive_decr.trais(db) {
//                     let trai_term = declarative_term_region.expr_term(trai.expr())?;
//                     let template = DeriveDecrDeclarativeSignatureTemplate::new(db, trai_term);
//                     // trai_term.a
//                     let trai_path = match trai_term {
//                         DeclarativeTerm::Literal(_) => todo!(),
//                         DeclarativeTerm::Symbol(_) => todo!(),
//                         DeclarativeTerm::Variable(_) => todo!(),
//                         DeclarativeTerm::EntityPath(path) => match path {
//                             DeclarativeTermEntityPath::Fugitive(_) => todo!(),
//                             DeclarativeTermEntityPath::Trait(trai_path) => trai_path,
//                             DeclarativeTermEntityPath::Type(_) => todo!(),
//                             DeclarativeTermEntityPath::TypeVariant(_) => todo!(),
//                         },
//                         DeclarativeTerm::Category(_) => todo!(),
//                         DeclarativeTerm::Universe(_) => todo!(),
//                         DeclarativeTerm::Curry(_) => todo!(),
//                         DeclarativeTerm::Ritchie(_) => todo!(),
//                         DeclarativeTerm::Abstraction(_) => todo!(),
//                         DeclarativeTerm::ExplicitApplication(_) => todo!(),
//                         DeclarativeTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
//                         DeclarativeTerm::Subitem(_) => todo!(),
//                         DeclarativeTerm::AsTraitSubitem(_) => todo!(),
//                         DeclarativeTerm::TraitConstraint(_) => todo!(),
//                         DeclarativeTerm::LeashOrBitNot(_) => todo!(),
//                         DeclarativeTerm::Wrapper(_) => todo!(),
//                         DeclarativeTerm::List(_) => todo!(),
//                     };
//                     map.update_value_or_insert(
//                         trai_path,
//                         |_| todo!(),
//                         Ok(SmallVecSet::new_one_elem_set(template)),
//                     )
//                 }
//             }
//         }
//     }
//     Ok(map)
// }
