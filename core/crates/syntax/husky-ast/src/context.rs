mod struct_item_context;

use husky_entity_syntax::EntitySyntaxQueryGroup;
pub use struct_item_context::*;

use crate::*;
use husky_file::FilePtr;
use husky_word::Paradigm;
use thin_vec::thin_vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReturnKind {
    Normal,
    Feature,
    LazyField,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstContext {
    Package(FilePtr),
    Module(EntityRoutePtr),
    Stmt {
        paradigm: Paradigm,
        return_kind: ReturnKind,
    },
    Match {
        paradigm: Paradigm,
        return_kind: ReturnKind,
    },
    Visual,
    Struct {
        opt_base_ty: Option<EntityRoutePtr>,
        item_context: StructItemContext,
    },
    Record,
    Enum(EntityRoutePtr),
}

impl AstContext {
    pub fn opt_subroute(
        self,
        db: &dyn EntitySyntaxQueryGroup,
        ident: CustomIdentifier,
    ) -> Option<EntityRoutePtr> {
        Some(match self {
            AstContext::Package(main) => db.subroute(db.module(main).unwrap(), ident, thin_vec![]),
            AstContext::Module(route) => db.subroute(route, ident, thin_vec![]),
            AstContext::Struct { opt_base_ty, .. } => db.subroute(opt_base_ty?, ident, thin_vec![]),
            AstContext::Enum(_) => todo!(),
            AstContext::Record => todo!(),
            _ => return None,
        })
    }

    pub fn return_kind(&self) -> ReturnKind {
        match self {
            AstContext::Stmt { return_kind, .. } => *return_kind,
            AstContext::Match { return_kind, .. } => *return_kind,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for AstContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AstContext::Package(_) => "package",
            AstContext::Module(_) => "module",
            AstContext::Stmt { .. } => "stmt",
            AstContext::Visual => "visual",
            AstContext::Struct { .. } => "struct",
            AstContext::Enum(_) => "enum",
            AstContext::Record => "record",
            AstContext::Match { .. } => todo!(),
        })
    }
}
