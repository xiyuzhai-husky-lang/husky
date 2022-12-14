mod struct_item_context;

pub use struct_item_context::*;

use crate::*;
use husky_package_path::PackagePath;
use husky_token::ParadigmKeyword;
use thin_vec::thin_vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RawReturnContext {
    pub opt_return_ty: Option<ExprIdx>,
    pub kind: RawReturnContextKind,
}

impl RawReturnContext {
    pub fn kind(&self) -> RawReturnContextKind {
        self.kind
    }

    pub fn return_ty(&self) -> Term {
        todo!()
        // self.opt_return_ty.unwrap().route
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawReturnContextKind {
    Normal,
    Feature,
    MemoField,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstContext {
    Package(PackagePath),
    Module(Term),
    Stmt {
        paradigm: ParadigmKeyword,
        return_context: Option<RawReturnContext>,
    },
    Match {
        paradigm: ParadigmKeyword,
        return_context: Option<RawReturnContext>,
    },
    Visual,
    Struct {
        opt_base_ty: Option<Term>,
        item_context: StructItemContext,
    },
    Record,
    Enum(Term),
}

impl AstContext {
    pub fn opt_subroute(self, db: &dyn AstDb, ident: Identifier) -> Option<Term> {
        todo!()
        // Some(match self {
        //     AstContext::Package(main) => db.subroute(db.module(main).unwrap(), ident, thin_vec![]),
        //     AstContext::Module(route) => db.subroute(route, ident, thin_vec![]),
        //     AstContext::Struct { opt_base_ty, .. } => db.subroute(opt_base_ty?, ident, thin_vec![]),
        //     AstContext::Enum(_) => todo!(),
        //     AstContext::Record => todo!(),
        //     _ => return None,
        // })
    }

    pub fn return_context(&self) -> Option<RawReturnContext> {
        match self {
            AstContext::Stmt { return_context, .. } => *return_context,
            AstContext::Match { return_context, .. } => *return_context,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for AstContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AstContext::Package(_) => "package",
            AstContext::Module { .. } => "module",
            AstContext::Stmt { .. } => "stmt",
            AstContext::Visual => "visual",
            AstContext::Struct { .. } => "struct",
            AstContext::Enum(_) => "enum",
            AstContext::Record => "record",
            AstContext::Match { .. } => todo!(),
        })
    }
}
