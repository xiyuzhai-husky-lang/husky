use std::sync::Arc;

use crate::*;
use ast::{Ast, AstIter, AstKind, AstQueryGroup, RawExprArena};
use entity_route_query::EntityRouteQueryGroup;
use file::FilePtr;
use infer_total::InferQueryGroup;

#[salsa::query_group(HighlightQueryGroupStorage)]
pub trait HighlightQueryGroup: EntityRouteQueryGroup + AstQueryGroup + InferQueryGroup {
    fn highlights(&self, file: FilePtr) -> Arc<Vec<Highlight>>;
}

fn highlights(db: &dyn HighlightQueryGroup, file: FilePtr) -> Arc<Vec<Highlight>> {
    let ast_text = db.ast_text(file).unwrap();
    let mut highlights = Vec::new();
    use fold::FoldStorage;
    collect_highlights(
        ast_text.folded_results.fold_iter(0),
        &ast_text.arena,
        &mut highlights,
    );
    Arc::new(highlights)
}

fn collect_highlights(ast_iter: AstIter, arena: &RawExprArena, highlights: &mut Vec<Highlight>) {
    for item in ast_iter {
        match item.value {
            Ok(ast) => collect_highlights_from_ast(ast, arena, highlights),
            Err(_) => (),
        }
        match item.children {
            Some(children) => collect_highlights(children, arena, highlights),
            None => todo!(),
        }
    }
}

fn collect_highlights_from_ast(ast: &Ast, arena: &RawExprArena, highlights: &mut Vec<Highlight>) {
    match ast.kind {
        AstKind::TypeDefnHead {
            ident,
            kind,
            ref generic_placeholders,
        } => todo!(),
        AstKind::MainDefn => todo!(),
        AstKind::RoutineDefnHead(ref head) => todo!(),
        AstKind::PatternDefnHead => todo!(),
        AstKind::FeatureDecl { ident, ty } => todo!(),
        AstKind::TypeMethodDefnHead(_) => todo!(),
        AstKind::FieldDefnHead(_) => todo!(),
        AstKind::DatasetConfigDefnHead => todo!(),
        AstKind::Stmt(_) => todo!(),
        AstKind::EnumVariantDefnHead {
            ident,
            variant_class,
        } => todo!(),
        AstKind::Use { ref use_variant } => todo!(),
    }
}
