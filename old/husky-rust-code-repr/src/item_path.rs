#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityRouteRole {
    Caller,
    StaticCallRoute,
    Decl,
    Other,
    ForAnyLifetimeOther,
    StaticDecl,
    StaticThinFpTyDecl { needs_eval_context: bool },
    FpValue,
}

impl EntityRouteRole {
    pub fn parent_role(self) -> Self {
        match self {
            EntityRouteRole::Caller => EntityRouteRole::Caller,
            EntityRouteRole::StaticCallRoute => EntityRouteRole::StaticCallRoute,
            EntityRouteRole::Decl => EntityRouteRole::Decl,
            EntityRouteRole::Other => EntityRouteRole::Other,
            EntityRouteRole::ForAnyLifetimeOther => EntityRouteRole::ForAnyLifetimeOther,
            EntityRouteRole::StaticDecl => EntityRouteRole::StaticDecl,
            EntityRouteRole::StaticThinFpTyDecl { .. } => panic!(),
            EntityRouteRole::FpValue => EntityRouteRole::Other,
        }
    }

    pub fn argument_role(self) -> Self {
        match self {
            EntityRouteRole::Caller => EntityRouteRole::Other,
            EntityRouteRole::StaticCallRoute | EntityRouteRole::ForAnyLifetimeOther => {
                EntityRouteRole::ForAnyLifetimeOther
            }
            EntityRouteRole::Decl => EntityRouteRole::Decl,
            EntityRouteRole::Other => EntityRouteRole::Other,
            EntityRouteRole::StaticDecl => EntityRouteRole::StaticDecl,
            EntityRouteRole::StaticThinFpTyDecl { .. } => EntityRouteRole::StaticDecl,
            EntityRouteRole::FpValue => todo!(),
        }
    }
}
