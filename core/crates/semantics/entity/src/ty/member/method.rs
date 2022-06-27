use avec::Avec;
use entity_route::EntityRouteKind;
use infer_decl::MethodKind;
use static_defn::MethodStaticDefnVariant;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MethodDefnVariant {
    TypeMethod {
        ty: EntityRoutePtr,
        method_source: CallFormSource,
    },
    TraitMethod {
        trai: EntityRoutePtr,
        opt_default_source: Option<CallFormSource>,
    },
    TraitMethodImpl {
        trai: EntityRoutePtr,
        opt_source: Option<CallFormSource>,
    },
}

impl MethodDefnVariant {
    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        method_kind: &MethodStaticDefnVariant,
    ) -> Self {
        match method_kind {
            MethodStaticDefnVariant::TypeMethod { source } => MethodDefnVariant::TypeMethod {
                ty: symbol_context.opt_this_ty().unwrap(),
                method_source: CallFormSource::Static(source.clone()),
            },
            MethodStaticDefnVariant::TraitMethod { opt_default_source } => {
                MethodDefnVariant::TraitMethod {
                    trai: symbol_context.trai(),
                    opt_default_source: opt_default_source
                        .as_ref()
                        .map(|source| CallFormSource::Static(source.clone())),
                }
            }
            MethodStaticDefnVariant::TraitMethodImpl { opt_source } => {
                panic!("this shouldn't be handled here")
            }
        }
    }
}
