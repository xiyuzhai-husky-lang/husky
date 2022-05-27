mod struct_item_context;

pub use struct_item_context::*;

use file::FilePtr;
use word::Paradigm;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstContext {
    Package(FilePtr),
    Module(EntityRoutePtr),
    Stmt(Paradigm),
    Match(Paradigm),
    Visual,
    Struct(StructItemContext),
    Record,
    Props,
    Enum(EntityRoutePtr),
}

impl AstContext {
    pub fn subroute(&self, db: &dyn AstSalsaQueryGroup, ident: CustomIdentifier) -> EntityRoutePtr {
        match self {
            AstContext::Package(main) => db.make_subroute(db.module(*main).unwrap(), ident, vec![]),
            AstContext::Module(route) => db.make_subroute(*route, ident, Vec::new()),
            AstContext::Stmt(_) => todo!(),
            AstContext::Match(_) => todo!(),
            AstContext::Struct(item_context) => todo!(),
            AstContext::Enum(_) => todo!(),
            AstContext::Record => todo!(),
            AstContext::Props => todo!(),
            AstContext::Visual => todo!(),
        }
    }
}

impl From<Paradigm> for AstContext {
    fn from(paradigm: Paradigm) -> Self {
        match paradigm {
            Paradigm::Procedural => AstContext::Stmt(Paradigm::Procedural),
            Paradigm::EagerFunctional => AstContext::Stmt(Paradigm::EagerFunctional),
            Paradigm::LazyFunctional => todo!(),
        }
    }
}

impl std::fmt::Display for AstContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AstContext::Package(_) => "package",
            AstContext::Module(_) => "module",
            AstContext::Stmt(Paradigm::LazyFunctional) => "def",
            AstContext::Stmt(Paradigm::EagerFunctional) => "func",
            AstContext::Stmt(Paradigm::Procedural) => "proc",
            AstContext::Visual => "visual",
            AstContext::Struct(_) => "struct",
            AstContext::Enum(_) => "enum",
            AstContext::Record => "record",
            AstContext::Props => "props",
            AstContext::Match(_) => todo!(),
        })
    }
}
