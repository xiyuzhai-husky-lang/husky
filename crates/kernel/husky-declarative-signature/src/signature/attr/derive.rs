use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveAttrDeclarativeSignatureTemplate {
    pub shards: SmallVec<[DeriveAttrShardDeclarativeSignatureTemplate; 8]>,
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveAttrShardDeclarativeSignatureTemplate {
    pub trai_term: DeclarativeTerm,
}

impl DeriveAttrDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        decl: DeriveAttrSynDecl,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let shards = decl
            .trais(db)
            .iter()
            .map(|trai| {
                Ok(DeriveAttrShardDeclarativeSignatureTemplate::new(
                    db,
                    declarative_term_region.expr_term(trai.syn_expr_idx())?,
                ))
            })
            .collect::<DeclarativeTermResultBorrowed2<_>>()?;
        Ok(DeriveAttrDeclarativeSignatureTemplate::new(db, shards))
    }
}

// // todo: change to ordered map and set
//     for attr_path in ty_path.attr_paths(db) {
//         match attr_path.syn_decl(db)? {
//             AttrSynDecl::Derive(derive_attr) => {
//                 let syn_expr_region = derive_attr.syn_expr_region(db);
//                 let declarative_term_region = declarative_term_region(db, syn_expr_region);
//                 for trai in derive_attr.trais(db) {
//                     let trai_term = declarative_term_region.expr_term(trai.expr())?;
//                     let template = DeriveAttrDeclarativeSignatureTemplate::new(db, trai_term);
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
