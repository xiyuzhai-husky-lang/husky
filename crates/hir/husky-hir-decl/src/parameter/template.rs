use crate::*;
use husky_hir_ty::trai::HirTrait;

use husky_syn_expr::{TemplateParameterSyndicateData, TemplateSynParameterData};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub struct HirTemplateParameter {
    symbol: HirComptimeSymbol,
    data: HirTemplateParameterData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db]
pub enum HirTemplateParameterData {
    Type { ident: Ident, traits: Vec<HirTrait> },
    Constant { ident: Ident, ty: HirType },
    Lifetime { label: Label },
    Place { label: Label },
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
        let symbol = HirComptimeSymbol::from_ethereal(symbol, db)?;
        let data = match syndicate.data() {
            TemplateParameterSyndicateData::Type {
                ident_token,
                traits,
            } => HirTemplateParameterData::Type {
                ident: ident_token.ident(),
                traits: match traits {
                    Some(_) =>
                    /* ad hoc */
                    {
                        vec![]
                    }
                    None => vec![],
                },
            },
            &TemplateParameterSyndicateData::Constant {
                const_token: _,
                ident_token,
                colon_token: _,
                ty_expr,
            } => {
                let ident = ident_token.ident();
                if ident.data(db) == "label" {
                    todo!()
                }
                HirTemplateParameterData::Constant {
                    ident,
                    ty: builder.hir_ty(ty_expr),
                }
            }
            TemplateParameterSyndicateData::Lifetime { label_token } => {
                HirTemplateParameterData::Lifetime {
                    label: label_token.label(),
                }
            }
            TemplateParameterSyndicateData::Place { label_token } => {
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

impl std::ops::Deref for HirTemplateParameters {
    type Target = [HirTemplateParameter];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HirTemplateParameterStats {
    pub tys: u8,
    pub constants: u8,
    pub lifetimes: u8,
    pub places: u8,
}

// #[salsa::tracked(jar = HirDeclJar)]
pub fn item_hir_template_parameter_stats(
    db: &::salsa::Db,
    item_path: ItemPath,
) -> Option<HirTemplateParameterStats> {
    let mut stats = HirTemplateParameterStats {
        tys: 0,
        constants: 0,
        lifetimes: 0,
        places: 0,
    };
    let hir_decl = item_path.hir_decl(db)?;
    for param in hir_decl.template_parameters(db) {
        match param.data {
            HirTemplateParameterData::Type { .. } => stats.tys += 1,
            HirTemplateParameterData::Constant { .. } => stats.constants += 1,
            HirTemplateParameterData::Lifetime { .. } => stats.lifetimes += 1,
            HirTemplateParameterData::Place { .. } => stats.places += 1,
        }
    }
    Some(stats)
}
