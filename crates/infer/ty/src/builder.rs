mod impl_expr;
mod impl_routine;
mod impl_stmt;

use super::*;
use ast::{AstIter, AstText};
use scope::InputPlaceholder;
use std::sync::Arc;
use text::TextRanged;

pub struct TySheetBuilder<'a> {
    db: &'a dyn InferTySalsaQueryGroup,
    main_file: FilePtr,
    ty_sheet: TySheet,
}

impl<'a> TySheetBuilder<'a> {
    pub(super) fn new(db: &'a dyn InferTySalsaQueryGroup, ast_text: Arc<AstText>) -> Self {
        Self {
            db,
            main_file: db.main_file(ast_text.file).unwrap(),
            ty_sheet: TySheet::new(ast_text),
        }
    }

    pub(super) fn finish(self) -> TySheet {
        self.ty_sheet
    }

    pub(super) fn infer_all(&mut self, ast_iter: AstIter) {
        let arena = self.ty_sheet.ast_text.arena.clone();
        for item in ast_iter {
            match item.value {
                Ok(value) => match value.kind {
                    AstKind::TypeDef { .. } | AstKind::EnumVariant { .. } => {
                        item.children.map(|children| self.infer_all(children));
                    }
                    AstKind::MainDecl => {
                        let output_ty = self.db.package_output_ty(self.main_file).unwrap();
                        self.infer_def(item.idx, &[], output_ty, item.children.unwrap(), &arena)
                    }
                    AstKind::DatasetConfig => self.infer_routine(
                        item.idx,
                        &[],
                        BuiltinIdentifier::DatasetType.into(),
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::RoutineDecl {
                        ref routine_head, ..
                    } => self.infer_routine(
                        item.idx,
                        &routine_head.input_placeholders,
                        routine_head.output.scope,
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::PatternDef => todo!(),
                    AstKind::Use { ident, scope } => todo!(),
                    AstKind::MembVar { .. } => (),
                    AstKind::Stmt(_) => todo!(),
                    AstKind::MembRoutineDecl {
                        ref memb_routine_head,
                        ..
                    } => self.infer_routine(
                        item.idx,
                        &memb_routine_head.input_placeholders,
                        memb_routine_head.output.scope,
                        item.children.unwrap(),
                        &arena,
                    ),
                },
                _ => (),
            }
        }
    }

    fn infer_def(
        &mut self,
        line_group_idx: usize,
        inputs: &[InputPlaceholder],
        output_ty: ScopePtr,
        ast_iter: AstIter,
        arena: &RawExprArena,
    ) {
        self.add_inputs(inputs);
        self.infer_stmts(ast_iter.clone(), output_ty, arena);
    }

    fn add_inputs(&mut self, inputs: &[InputPlaceholder]) {
        if inputs.len() > 0 {
            if let None = self
                .ty_sheet
                .variables
                .get(&(inputs[0].ident, inputs[0].ranged_ty.row()))
            {
                for input in inputs {
                    should!(self
                        .ty_sheet
                        .variables
                        .insert(
                            (input.ident, inputs[0].ranged_ty.row()),
                            Some(input.ranged_ty.scope),
                        )
                        .is_none());
                }
            }
        }
    }
}
