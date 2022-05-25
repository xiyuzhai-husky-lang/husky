use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, GenericArgument};
use entity_syntax::EntityRouteSalsaQueryGroup;
use print_utils::p;
use word::CustomIdentifier;

pub struct Implementor<'a> {
    db: &'a dyn EntityRouteSalsaQueryGroup,
    this_ty: EntityRoutePtr,
    member_impls: &'a [(CustomIdentifier, GenericArgument)],
}

impl<'a> Implementor<'a> {
    pub fn new(
        db: &'a dyn EntityRouteSalsaQueryGroup,
        this_ty: EntityRoutePtr,
        member_impls: &'a [(CustomIdentifier, GenericArgument)],
    ) -> Self {
        Self {
            db,
            this_ty,
            member_impls,
        }
    }

    pub fn generic_argument(&self, ident0: CustomIdentifier) -> GenericArgument {
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
                    (ty.kind, ty.generic_arguments.clone())
                }
                _ => todo!(),
            },
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, entity_kind } => todo!(),
            EntityRouteKind::ThisType => (
                implementor.this_ty.kind,
                implementor.this_ty.generic_arguments.clone(),
            ),
            EntityRouteKind::TypeAsTraitMember {
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
