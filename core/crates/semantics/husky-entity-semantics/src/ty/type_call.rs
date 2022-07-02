use defn_head::Parameter;
use husky_atom::AtomContext;
use husky_entity_route_syntax::RangedEntityRoute;
use map_collect::MapCollect;
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant, FunctionStaticDefnVariant};
use std::sync::Arc;
use vm::{GenericRoutineLinkage, SpecificRoutineLinkage, __Linkage};

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
                __Linkage,
                ..
            } => TypeCallDefn {
                parameters: Arc::new(
                    parameters
                        .map(|input_placeholder| context.parameter_from_static(input_placeholder)),
                ),
                output_ty: RangedEntityRoute {
                    route: context.parse_entity_route(output_ty).unwrap(),
                    range: Default::default(),
                },
                opt_linkage: Some(__Linkage),
            },
            _ => panic!(),
        })
    }
}
