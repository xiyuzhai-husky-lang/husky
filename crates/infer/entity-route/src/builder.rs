mod impl_expr;
mod impl_locality;
mod impl_morphism;
mod impl_routine;
mod impl_stmt;

use super::*;
use ast::{AstIter, AstText};
use fold::LocalStack;
use std::sync::Arc;
use text::TextRanged;

pub struct TySheetBuilder<'a> {
    db: &'a dyn InferTyQueryGroup,
    main_file: FilePtr,
    ty_sheet: EntityRouteSheet,
    trait_uses: LocalStack<EntityRouteKind>,
}

impl<'a> TySheetBuilder<'a> {
    pub(super) fn new(db: &'a dyn InferTyQueryGroup, ast_text: Arc<AstText>) -> Self {
        Self {
            db,
            main_file: db.main_file(ast_text.file).unwrap(),
            ty_sheet: EntityRouteSheet::new(ast_text),
            trait_uses: LocalStack::new(),
        }
    }

    pub(super) fn finish(self) -> EntityRouteSheet {
        self.ty_sheet
    }

    pub(super) fn infer_all(&mut self, ast_iter: AstIter) {
        self.enter_block();
        let arena = self.ty_sheet.ast_text.arena.clone();
        for item in ast_iter {
            match item.value {
                Ok(value) => match value.kind {
                    AstKind::TypeDefnHead { .. } | AstKind::EnumVariantDefnHead { .. } => {
                        item.children.map(|children| self.infer_all(children));
                    }
                    AstKind::MainDefn => {
                        let output_ty = self.db.global_output_ty(self.main_file).unwrap();
                        self.infer_morphism(&[], output_ty, item.children.unwrap(), &arena)
                    }
                    AstKind::DatasetConfigDefnHead => self.infer_routine(
                        &[],
                        RootIdentifier::DatasetType.into(),
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::RoutineDefnHead(ref head) => self.infer_routine(
                        &head.input_placeholders,
                        head.output_ty.route,
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::PatternDefnHead => todo!(),
                    AstKind::Use { .. } => (),
                    AstKind::FieldDefnHead(ref head) => match head.kind {
                        FieldKind::StructOriginal => (),
                        FieldKind::RecordOriginal => (),
                        FieldKind::StructDerived | FieldKind::RecordDerived => {
                            self.infer_morphism(&[], head.ty, item.children.unwrap(), &arena)
                        }
                    },
                    AstKind::Stmt(_) => todo!(),
                    AstKind::TypeMethodDefnHead(ref head) => self.infer_routine(
                        &head.input_placeholders,
                        head.output_ty.route,
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::FeatureDecl { ty, .. } => {
                        self.infer_morphism(&[], ty.route, item.children.unwrap(), &arena)
                    }
                },
                _ => (),
            }
        }
        self.exit_block()
    }

    fn add_inputs(&mut self, inputs: &[InputPlaceholder]) {
        if inputs.len() > 0 {
            if let None = self
                .ty_sheet
                .variable_tys
                .get(&(inputs[0].ident, inputs[0].ranged_ty.row()))
            {
                for input in inputs {
                    should!(self
                        .ty_sheet
                        .variable_tys
                        .insert(
                            (input.ident, inputs[0].ranged_ty.row()),
                            Some(input.ranged_ty.route),
                        )
                        .is_none());
                }
            }
        }
    }
}
