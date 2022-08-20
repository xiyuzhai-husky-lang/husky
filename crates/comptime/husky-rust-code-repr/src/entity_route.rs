use super::*;
use husky_entity_route::EntityRoutePtr;

pub struct EntityRouteRepr<'a> {
    route: EntityRoutePtr,
    role: EntityRouteRole,
    entity_route_uses: &'a [EntityRoutePtr],
}

impl<'a> std::fmt::Display for EntityRouteRepr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityRouteRole {
    Caller,
    StaticCallRoute,
    Decl,
    Other,
    StaticOther,
}

impl EntityRouteRole {
    pub fn parent_role(self) -> Self {
        match self {
            EntityRouteRole::Caller => EntityRouteRole::Caller,
            EntityRouteRole::StaticCallRoute => EntityRouteRole::StaticCallRoute,
            EntityRouteRole::Decl => EntityRouteRole::Decl,
            EntityRouteRole::Other => EntityRouteRole::Other,
            EntityRouteRole::StaticOther => EntityRouteRole::StaticOther,
        }
    }

    pub fn argument_role(self) -> Self {
        match self {
            EntityRouteRole::Caller => EntityRouteRole::Other,
            EntityRouteRole::StaticCallRoute | EntityRouteRole::StaticOther => {
                EntityRouteRole::StaticOther
            }
            EntityRouteRole::Decl => EntityRouteRole::Decl,
            EntityRouteRole::Other => EntityRouteRole::Other,
        }
    }
}
