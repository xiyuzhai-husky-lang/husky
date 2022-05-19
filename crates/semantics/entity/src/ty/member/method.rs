use avec::Avec;
use entity_route::EntityRouteKind;
use infer_decl::MethodKind;
use static_defn::{LinkageSource, MethodStaticDefnVariant};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MethodSource {
    Func { stmts: Avec<FuncStmt> },
    Proc { stmts: Avec<ProcStmt> },
    Pattern { stmts: Avec<LazyStmt> },
    Static(LinkageSource),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MethodDefnVariant {
    TypeMethod {
        ty: EntityRoutePtr,
        method_source: MethodSource,
    },
    TraitMethod {
        trai: EntityRoutePtr,
        opt_default_source: Option<MethodSource>,
    },
    TraitMethodImpl {
        trai: EntityRoutePtr,
        opt_source: Option<MethodSource>,
    },
}

impl MethodDefnVariant {
    pub fn from_static(
        symbol_context: &SymbolContext,
        method_kind: &MethodStaticDefnVariant,
    ) -> Self {
        match method_kind {
            MethodStaticDefnVariant::TypeMethod { source } => MethodDefnVariant::TypeMethod {
                ty: symbol_context.opt_this_ty.unwrap(),
                method_source: MethodSource::Static(source.clone()),
            },
            MethodStaticDefnVariant::TraitMethod { opt_default_source } => {
                MethodDefnVariant::TraitMethod {
                    trai: symbol_context.trai(),
                    opt_default_source: opt_default_source
                        .as_ref()
                        .map(|source| MethodSource::Static(source.clone())),
                }
            }
            MethodStaticDefnVariant::TraitMethodImpl { opt_source } => {
                panic!("this shouldn't be handled here")
            }
        }
    }
}
