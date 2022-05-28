use atom::AtomContext;
use defn_head::InputParameter;
use entity_route::RangedEntityRoute;
use map_collect::MapCollect;
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use std::sync::Arc;
use vm::Linkage;

#[derive(Debug, PartialEq, Eq)]
pub struct TyCallDefn {
    pub parameters: Arc<Vec<InputParameter>>,
    pub output_ty: RangedEntityRoute,
    pub source: TyCallSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TyCallSource {
    GenericStruct,
    GenericRecord,
    Static(Linkage),
}

impl TyCallDefn {
    pub fn from_static(
        context: &mut dyn AtomContext,
        static_defn: &EntityStaticDefn,
    ) -> Arc<TyCallDefn> {
        Arc::new(match static_defn.variant {
            EntityStaticDefnVariant::Routine {
                ref parameters,
                output_ty,
                linkage,
                ..
            } => TyCallDefn {
                parameters: Arc::new(parameters.map(|input_placeholder| {
                    context.input_placeholder_from_static(input_placeholder)
                })),
                output_ty: RangedEntityRoute {
                    route: context.entity_route_from_str(output_ty).unwrap(),
                    range: Default::default(),
                },
                source: TyCallSource::Static(linkage),
            },
            _ => panic!(),
        })
    }
}
