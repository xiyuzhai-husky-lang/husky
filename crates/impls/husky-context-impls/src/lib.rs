use husky_ast::{RawReturnContext, RawReturnContextKind};
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use infer_decl::DeclQueryGroup;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnContext {
    return_ty: EntityRoutePtr,
    kind: RawReturnContextKind,
}

impl ReturnContext {
    pub fn from_raw(db: &dyn DeclQueryGroup, raw: RawReturnContext) -> Self {
        Self {
            return_ty: { db.implement_target(raw.return_ty()).unwrap() },
            kind: raw.kind(),
        }
    }

    pub fn return_ty(&self) -> EntityRoutePtr {
        self.return_ty
    }
}
