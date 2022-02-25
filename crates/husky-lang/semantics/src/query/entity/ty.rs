use ast::{Ast, AstResult};
use fold::{FoldIter, FoldIterItem, FoldedList};
use syntax_types::GenericPlaceholderKind;

use crate::{error::err, kind::ty::TyKind, *};

pub(super) fn ty_from_ast(
    ident: CustomIdentifier,
    kind: &syntax_types::TyKind,
    generics: &[GenericPlaceholderKind],
    block: Option<FoldIter<AstResult<Ast>, FoldedList<AstResult<Ast>>>>,
    subentities: Arc<Vec<Arc<Entity>>>,
    scope: ScopePtr,
    file: FilePtr,
    vc: &EntityVersionControl,
) -> SemanticResultArc<Entity> {
    match kind {
        syntax_types::TyKind::Enum(_) => todo!(),
        syntax_types::TyKind::Struct => {
            struct_from_ast(ident, kind, generics, block, subentities, scope, file, vc)
        }
    }
}

pub(super) fn struct_from_ast(
    ident: CustomIdentifier,
    kind: &syntax_types::TyKind,
    generics: &[GenericPlaceholderKind],
    block: Option<FoldIter<AstResult<Ast>, FoldedList<AstResult<Ast>>>>,
    subentities: Arc<Vec<Arc<Entity>>>,
    scope: ScopePtr,
    file: FilePtr,
    vc: &EntityVersionControl,
) -> SemanticResultArc<Entity> {
    let block = if let Some(block) = block {
        block
    } else {
        todo!()
    };
    let mut memb_vars = Vec::new();
    for FoldIterItem { value, .. } in block {
        let ast = value.as_ref()?;
        match ast {
            Ast::TypeDef { .. } => (),
            Ast::MainDef | Ast::DatasetConfig => panic!(),
            Ast::FuncDef { kind, decl } => todo!(),
            Ast::PatternDef => panic!(),
            Ast::Use { .. } => (),
            Ast::MembDef { ident, kind } => match kind {
                syntax_types::MembKind::MembVar { ty } => {
                    memb_vars.push(crate::kind::ty::MembVar {
                        ident: *ident,
                        ty: ty.clone(),
                    })
                }
                syntax_types::MembKind::MembFunc {
                    this,
                    inputs,
                    output,
                    args,
                } => todo!(),
            },
            Ast::Stmt(_) => todo!(),
        }
    }
    Ok(Arc::new(Entity::new(
        ident,
        EntityKind::Ty(Ty {
            kind: TyKind::Struct { memb_vars },
        }),
        subentities,
        scope,
        file,
        vc,
    )))
}
