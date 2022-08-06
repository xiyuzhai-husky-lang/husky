use defn_head::Parameter;
use husky_atom::AtomContext;
use husky_entity_route::RangedEntityRoute;
use map_collect::MapCollect;
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant, FunctionStaticDefnVariant};
use std::sync::Arc;
use vm::{__Linkage, __LinkageFp};

#[derive(Debug, PartialEq, Eq)]
pub struct TypeCallDefn {
    pub parameters: Arc<Vec<Parameter>>,
    pub return_ty: RangedEntityRoute,
    pub opt_linkage: Option<__Linkage>,
}

impl TypeCallDefn {
    pub fn from_static(
        context: &mut dyn AtomContext,
        static_defn: &EntityStaticDefn,
    ) -> Arc<TypeCallDefn> {
        Arc::new(match static_defn.variant {
            EntityStaticDefnVariant::Function {
                ref parameters,
                return_ty,
                linkage,
                ..
            } => TypeCallDefn {
                parameters: Arc::new(
                    parameters.map(|parameter| context.parameter_from_static(parameter)),
                ),
                return_ty: RangedEntityRoute {
                    route: context.parse_entity_route(return_ty).unwrap(),
                    range: Default::default(),
                },
                opt_linkage: Some(linkage),
            },
            _ => panic!(),
        })
    }
}
