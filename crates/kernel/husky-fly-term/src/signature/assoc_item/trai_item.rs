pub mod assoc_ritchie;
pub mod method_ritchie;

use super::*;
use husky_coword::Ident;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_entity_tree::node::HasAssocItemPaths;
use husky_eth_signature::signature::{assoc_item::trai_item::TraitItemEthTemplate, HasEthTemplate};
use husky_eth_term::{instantiation::EthInstantiate, term::application::TermFunctionReduced};
use vec_like::VecMapGetEntry;

#[derive(Debug, PartialEq, Eq)]
pub enum TraitItemFlySignature {
    AssocType {
        path: TraitItemPath,
        self_ty: FlyTerm,
        trai: FlyTerm,
        item_ty: FlyTerm,
        instantiation: FlyInstantiation,
    },
    StaticVar {
        path: TraitItemPath,
        self_ty: FlyTerm,
        trai: FlyTerm,
        item_ty: FlyTerm,
        instantiation: FlyInstantiation,
    },
}

/// # constructor

impl TraitItemFlySignature {
    /// assuming that we already have that `self_ty` satisfies `trai`
    pub(crate) fn from_as_trai(
        self_ty: FlyTerm,
        trai: impl Into<FlyTerm>,
        ident: Ident,
        syn_expr_idx: SynExprIdx,
        engine: &mut impl FlyTermEngineMut,
    ) -> FlyTermResult<Self> {
        let db = engine.db();
        let trai = trai.into();
        let FlyTermData::Trait {
            trai_path,
            trai_arguments,
            ..
        } = trai.data(engine)
        else {
            todo!()
        };
        let Some(&(_, path)) = trai_path.assoc_item_paths(db).get_entry(ident) else {
            todo!()
        };
        let trai_eth_template = trai_path.eth_template(db)?;
        let eth_template = path.eth_template(db)?;
        let context_itd = engine.context_itd();
        let instantiation = FlyInstantiation::new_trai_item_instantiation_with_determined_trai(
            path,
            FlyInstantiationEnvironment::AssocType,
            syn_expr_idx,
            self_ty,
            trai_eth_template.template_parameters(db),
            trai_arguments.to_smallvec(),
            eth_template.template_parameters(db),
            engine.fly_terms_mut(),
            context_itd,
            db,
        );
        match eth_template {
            TraitItemEthTemplate::AssocRitchie(_) => todo!(),
            TraitItemEthTemplate::AssocVal(_) => todo!(),
            TraitItemEthTemplate::AssocType(_) => Ok(TraitItemFlySignature::AssocType {
                path,
                self_ty,
                trai: trai.into(),
                item_ty: engine.term_menu().ty0().into(),
                instantiation,
            }),
            TraitItemEthTemplate::AssocStaticMut(_) => todo!(),
            TraitItemEthTemplate::AssocStaticVar(eth_template) => {
                Ok(TraitItemFlySignature::AssocType {
                    path,
                    self_ty,
                    trai: trai.into(),
                    item_ty: eth_template.var_ty(db).into(),
                    instantiation,
                })
            }
            TraitItemEthTemplate::MethodRitchie(_) => todo!(),
            TraitItemEthTemplate::MethodCurry(_) => todo!(),
        }
    }
}

/// # getters

impl TraitItemFlySignature {
    pub fn self_ty(&self) -> FlyTerm {
        match *self {
            TraitItemFlySignature::AssocType { self_ty, .. } => self_ty,
            TraitItemFlySignature::StaticVar { self_ty, .. } => self_ty,
        }
    }

    pub fn trai(&self) -> FlyTerm {
        match *self {
            TraitItemFlySignature::AssocType { trai, .. } => trai,
            TraitItemFlySignature::StaticVar { trai, .. } => trai,
        }
    }

    pub fn path(&self) -> TraitItemPath {
        match *self {
            TraitItemFlySignature::AssocType { path, .. } => path,
            TraitItemFlySignature::StaticVar { path, .. } => path,
        }
    }

    pub fn item_ty(&self) -> FlyTerm {
        match *self {
            TraitItemFlySignature::AssocType { item_ty, .. } => item_ty,
            TraitItemFlySignature::StaticVar { item_ty, .. } => item_ty,
        }
    }
}
