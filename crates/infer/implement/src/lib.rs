use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, GenericArgument};
use entity_route_query::EntityRouteSalsaQueryGroup;

pub struct Implementor<'a> {
    pub db: &'a dyn EntityRouteSalsaQueryGroup,
    pub this_ty: EntityRoutePtr,
    // pub associated_type_placeholders,
    // pub dst_types
}

pub trait Implementable {
    type Target;

    fn implement(&self, implementor: &Implementor) -> Self::Target;
}

impl Implementable for EntityRoutePtr {
    type Target = Self;

    fn implement(&self, implementor: &Implementor) -> Self::Target {
        let (kind, mut generic_arguments) = match self.kind {
            EntityRouteKind::Root { ident } => todo!(),
            EntityRouteKind::Package { main, ident } => todo!(),
            EntityRouteKind::Child { parent, ident } => todo!(),
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, entity_kind } => todo!(),
            EntityRouteKind::ThisType => (
                implementor.this_ty.kind,
                implementor.this_ty.generic_arguments.clone(),
            ),
            EntityRouteKind::TraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        };
        for generic_argument in self.generic_arguments.iter() {
            generic_arguments.push(generic_argument.implement(implementor))
        }
        implementor.db.intern_entity_route(EntityRoute {
            kind,
            generic_arguments,
        })
    }
}

impl Implementable for GenericArgument {
    type Target = Self;

    fn implement(&self, implementor: &Implementor) -> Self::Target {
        match self {
            GenericArgument::Const(value) => GenericArgument::Const(*value),
            GenericArgument::EntityRoute(entity_route) => {
                GenericArgument::EntityRoute(entity_route.implement(implementor))
            }
        }
    }
}
