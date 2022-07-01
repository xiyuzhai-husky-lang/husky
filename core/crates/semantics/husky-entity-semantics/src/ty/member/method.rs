use avec::Avec;
use husky_entity_route_syntax::EntityRouteKind;
use infer_decl::MethodKind;
use static_defn::MethodStaticDefnKind;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodDefnKind {
    TypeMethod { ty: EntityRoutePtr },
    TraitMethod { trai: EntityRoutePtr },
    TraitMethodImpl { trai: EntityRoutePtr },
}

impl MethodDefnKind {
    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        method_kind: MethodStaticDefnKind,
    ) -> Self {
        match method_kind {
            MethodStaticDefnKind::TypeMethod => MethodDefnKind::TypeMethod {
                ty: symbol_context.opt_this_ty().unwrap(),
            },
            MethodStaticDefnKind::TraitMethod => MethodDefnKind::TraitMethod {
                trai: symbol_context.trai(),
            },
            MethodStaticDefnKind::TraitMethodImpl => {
                panic!("this shouldn't be handled here")
            }
        }
    }
}
