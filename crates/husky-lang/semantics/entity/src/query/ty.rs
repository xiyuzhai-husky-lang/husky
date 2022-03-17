use ast::{Ast, AstKind, AstResult};
use fold::{FoldIter, FoldIterItem, FoldedList};
use syntax_types::GenericPlaceholder;

use crate::{kind::ty::TyKind, *};
use semantics_error::*;

pub(super) fn ty_from_ast(
    ident: CustomIdentifier,
    kind: &syntax_types::TyKind,
    generics: &[GenericPlaceholder],
    block: Option<FoldIter<AstResult<Ast>, FoldedList<AstResult<Ast>>>>,
    subentities: Arc<Vec<Arc<Entity>>>,
    scope: ScopePtr,
    file: FilePtr,
    range: TextRange,
    vc: &EntityVersionControl,
) -> SemanticResultArc<Entity> {
    match kind {
        syntax_types::TyKind::Enum(_) => todo!(),
        syntax_types::TyKind::Struct => struct_from_ast(
            ident,
            kind,
            generics,
            block,
            subentities,
            scope,
            file,
            range,
            vc,
        ),
    }
}

pub(super) fn struct_from_ast(
    ident: CustomIdentifier,
    kind: &syntax_types::TyKind,
    generics: &[GenericPlaceholder],
    block: Option<FoldIter<AstResult<Ast>, FoldedList<AstResult<Ast>>>>,
    subentities: Arc<Vec<Arc<Entity>>>,
    scope: ScopePtr,
    file: FilePtr,
    range: TextRange,
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
        match ast.kind {
            AstKind::TypeDef { .. } => (),
            AstKind::MainDef | AstKind::DatasetConfig => panic!(),
            AstKind::RoutineDef {
                routine_kind: ref kind,
                routine_head: ref decl,
            } => todo!(),
            AstKind::PatternDef => panic!(),
            AstKind::Use { .. } => (),
            AstKind::MembDef {
                ident,
                memb_kind: ref kind,
            } => match kind {
                syntax_types::MembKind::MembVar { ty } => {
                    memb_vars.push(crate::kind::ty::MembVar {
                        ident,
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
            AstKind::Stmt(_) => todo!(),
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
        range,
        vc,
    )))
}
