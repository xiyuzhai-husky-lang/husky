use husky_atom::AtomContext;
use husky_defn_head::Parameter;
use husky_entity_route::RangedEntityRoute;
use husky_static_defn::{EntityStaticDefn, EntityStaticDefnVariant, FunctionStaticDefnVariant};
use map_collect::MapCollect;
use std::sync::Arc;
use vm::{__Linkage, __ResolvedLinkage};

#[derive(Debug, PartialEq, Eq)]
pub struct TypeCallDefn {
    pub parameters: Arc<Vec<Parameter>>,
    pub output_ty: RangedEntityRoute,
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
                output_ty,
                linkage,
                ..
            } => TypeCallDefn {
                parameters: Arc::new(
                    parameters.map(|parameter| context.parameter_from_static(parameter)),
                ),
                output_ty: RangedEntityRoute {
                    route: context.parse_entity_route(output_ty).unwrap(),
                    range: Default::default(),
                },
                opt_linkage: Some(linkage),
            },
            _ => panic!(),
        })
    }
}
