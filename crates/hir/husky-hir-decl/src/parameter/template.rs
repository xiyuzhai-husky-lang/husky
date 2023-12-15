use crate::*;
use husky_fluffy_term::FluffyTermBase;
use husky_hir_ty::trai::HirTrait;

use husky_syn_expr::{
    trais::TraitsSyndicate, TemplateParameterSyndicateVariant, TemplateSynParameterData,
};
use smallvec::SmallVec;

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirTemplateParameter {
    symbol: HirTemplateSymbol,
    data: HirTemplateParameterData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirTemplateParameterData {
    Type {
        ident: Ident,
        traits: SmallVec<[HirTrait; 4]>,
    },
    Constant {
        ident: Ident,
        ty: HirType,
    },
    Lifetime {
        label: Label,
    },
    Place {
        label: Label,
    },
}

impl HirTemplateParameter {
    pub fn from_syn(
        syndicate: &TemplateSynParameterData,
        builder: &HirDeclBuilder,
    ) -> Option<Self> {
        let EtherealTerm::Symbol(symbol) = builder.current_syn_symbol_term(syndicate.symbol())
        else {
            todo!()
        };
        let db = builder.db();
        let symbol = HirTemplateSymbol::from_ethereal(symbol, db)?;
        let data = match *syndicate.variant() {
            TemplateParameterSyndicateVariant::Type {
                ident_token,
                ref traits,
            } => HirTemplateParameterData::Type {
                ident: ident_token.ident(),
                traits: match *traits {
                    Some(TraitsSyndicate {
                        ref trait_syn_expr_idxs,
                        ..
                    }) => trait_syn_expr_idxs
                        .iter()
                        .map(|&trai_syn_expr_idx| {
                            let sema_expr_region_data = &builder.sema_expr_region_data();
                            let terms = sema_expr_region_data.fluffy_term_region().terms();
                            let trai_term = match sema_expr_region_data
                                .syn_root_expr_term(trai_syn_expr_idx)
                                .expect("some")
                                .expect("ok")
                                .base_resolved_inner(terms)
                            {
                                FluffyTermBase::Ethereal(trai_term) => trai_term,
                                FluffyTermBase::Solid(_) => todo!(),
                                FluffyTermBase::Hollow(_) => todo!(),
                                FluffyTermBase::Place => todo!(),
                            };
                            HirTrait::from_ethereal(trai_term, db)
                        })
                        .collect(),
                    None => smallvec![],
                },
            },
            TemplateParameterSyndicateVariant::Constant {
                ident_token,
                ty_expr,
                ..
            } => {
                let ident = ident_token.ident();
                HirTemplateParameterData::Constant {
                    ident,
                    ty: builder.hir_ty(ty_expr).unwrap(),
                }
            }
            TemplateParameterSyndicateVariant::Lifetime { label_token } => {
                HirTemplateParameterData::Lifetime {
                    label: label_token.label(),
                }
            }
            TemplateParameterSyndicateVariant::Place { label_token } => {
                HirTemplateParameterData::Place {
                    label: label_token.label(),
                }
            }
        };
        Some(Self { data, symbol })
    }

    pub fn data(&self) -> &HirTemplateParameterData {
        &self.data
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirTemplateParameters(SmallVec<[HirTemplateParameter; 2]>);

impl HirTemplateParameters {
    pub fn from_syn(syndicates: &[TemplateSynParameterData], builder: &HirDeclBuilder) -> Self {
        HirTemplateParameters(
            syndicates
                .iter()
                .filter_map(|syndicate| HirTemplateParameter::from_syn(syndicate, builder))
                .collect(),
        )
    }
}

impl<'a> std::iter::IntoIterator for &'a HirTemplateParameters {
    type Item = &'a HirTemplateParameter;

    type IntoIter = impl Iterator<Item = &'a HirTemplateParameter> + 'a;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl std::ops::Deref for HirTemplateParameters {
    type Target = [HirTemplateParameter];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HirTemplateParameterStats {
    pub tys: u8,
    pub constants: u8,
    pub lifetimes: u8,
    pub places: u8,
}

#[salsa::tracked(jar = HirDeclJar)]
pub fn item_hir_template_parameter_stats(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> Option<HirTemplateParameterStats> {
    let item_path = item_path_id.item_path(db);
    let mut stats = HirTemplateParameterStats {
        tys: 0,
        constants: 0,
        lifetimes: 0,
        places: 0,
    };
    let hir_decl = item_path.hir_decl(db)?;
    let Some(template_parameters) = hir_decl.template_parameters(db) else {
        return Some(HirTemplateParameterStats::default());
    };
    for param in template_parameters {
        match param.data {
            HirTemplateParameterData::Type { .. } => stats.tys += 1,
            HirTemplateParameterData::Constant { .. } => stats.constants += 1,
            HirTemplateParameterData::Lifetime { .. } => stats.lifetimes += 1,
            HirTemplateParameterData::Place { .. } => stats.places += 1,
        }
    }
    Some(stats)
}
