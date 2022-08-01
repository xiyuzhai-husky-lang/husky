use super::*;

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

#[derive(Debug, Clone, Copy)]
pub enum EntityRouteRole {
    Caller,
    Decl,
    Other,
}

impl EntityRouteRole {
    pub fn argument_role(self) -> Self {
        match self {
            EntityRouteRole::Caller => EntityRouteRole::Other,
            EntityRouteRole::Decl => EntityRouteRole::Decl,
            EntityRouteRole::Other => EntityRouteRole::Other,
        }
    }
}
