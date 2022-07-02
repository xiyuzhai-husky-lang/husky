use husky_entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, SpatialArgument};
use husky_entity_syntax::EntitySyntaxSalsaQueryGroup;
use print_utils::p;
use thin_vec::thin_vec;
use word::CustomIdentifier;

pub struct Implementor<'a> {
    db: &'a dyn EntitySyntaxSalsaQueryGroup,
    this_ty: EntityRoutePtr,
    member_impls: &'a [(CustomIdentifier, SpatialArgument)],
}

impl<'a> Implementor<'a> {
    pub fn new(
        db: &'a dyn EntitySyntaxSalsaQueryGroup,
        this_ty: EntityRoutePtr,
        member_impls: &'a [(CustomIdentifier, SpatialArgument)],
    ) -> Self {
        Self {
            db,
            this_ty,
            member_impls,
        }
    }

    pub fn generic_argument(&self, ident0: CustomIdentifier) -> SpatialArgument {
        self.member_impls
            .iter()
            .find_map(|(ident, generic_argument)| {
                if *ident == ident0 {
                    Some(*generic_argument)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl<'a> std::fmt::Debug for Implementor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Implementor")
            .field("this_ty", &self.this_ty)
            .finish()
    }
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
            EntityRouteKind::Child { parent, ident } => match parent.kind {
                EntityRouteKind::ThisType => {
                    let ty = implementor.generic_argument(ident).take_entity_route();
                    (ty.kind, ty.spatial_arguments.clone())
                }
                _ => todo!(),
            },
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, entity_kind } => todo!(),
            EntityRouteKind::ThisType => (
                implementor.this_ty.kind,
                implementor.this_ty.spatial_arguments.clone(),
            ),
            EntityRouteKind::TypeAsTraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        };
        for generic_argument in self.spatial_arguments.iter() {
            generic_arguments.push(generic_argument.implement(implementor))
        }
        implementor.db.intern_entity_route(EntityRoute {
            kind,
            temporal_arguments: thin_vec![],
            spatial_arguments: generic_arguments,
        })
    }
}

impl Implementable for SpatialArgument {
    type Target = Self;

    fn implement(&self, implementor: &Implementor) -> Self::Target {
        match self {
            SpatialArgument::Const(value) => SpatialArgument::Const(*value),
            SpatialArgument::EntityRoute(entity_route) => {
                SpatialArgument::EntityRoute(entity_route.implement(implementor))
            }
        }
    }
}
