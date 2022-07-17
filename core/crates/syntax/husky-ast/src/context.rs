mod struct_item_context;

use husky_entity_syntax::EntitySyntaxQueryGroup;
pub use struct_item_context::*;

use crate::*;
use husky_file::FilePtr;
use thin_vec::thin_vec;
use word::Paradigm;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstContext {
    Package(FilePtr),
    Module(EntityRoutePtr),
    Stmt {
        paradigm: Paradigm,
        returns_feature: bool,
    },
    Match {
        paradigm: Paradigm,
        returns_feature: bool,
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
            AstContext::Package(main) => {
                db.make_subroute(db.module(main).unwrap(), ident, thin_vec![])
            }
            AstContext::Module(route) => db.make_subroute(route, ident, thin_vec![]),
            AstContext::Struct { opt_base_ty, .. } => {
                db.make_subroute(opt_base_ty?, ident, thin_vec![])
            }
            AstContext::Enum(_) => todo!(),
            AstContext::Record => todo!(),
            _ => return None,
        })
    }

    pub fn returns_feature(&self) -> bool {
        match self {
            AstContext::Stmt {
                returns_feature, ..
            } => *returns_feature,
            AstContext::Match {
                returns_feature, ..
            } => *returns_feature,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for AstContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AstContext::Package(_) => "package",
            AstContext::Module(_) => "module",
            AstContext::Stmt { .. } => todo!(),
            AstContext::Visual => "visual",
            AstContext::Struct { .. } => "struct",
            AstContext::Enum(_) => "enum",
            AstContext::Record => "record",
            AstContext::Match { .. } => todo!(),
        })
    }
}
