mod impl_expr;
mod impl_locality;
mod impl_morphism;
mod impl_routine;
mod impl_stmt;

use super::*;
use ast::{AstIter, AstText};
use entity_kind::FieldKind;
use fold::LocalStack;
use std::sync::Arc;
use text::TextRanged;

pub struct EntityRouteSheetBuilder<'a> {
    db: &'a dyn InferEntityRouteQueryGroup,
    main_file: FilePtr,
    entity_route_sheet: EntityRouteSheet,
    trait_uses: LocalStack<EntityRouteKind>,
}

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn new(db: &'a dyn InferEntityRouteQueryGroup, ast_text: Arc<AstText>) -> Self {
        let main_file = db.main_file(ast_text.file).unwrap();
        let mut global_errors = Vec::new();
        match db.global_input_ty(main_file) {
            Ok(_) => (),
            Err(e) => global_errors.push(e),
        }
        match db.global_output_ty(main_file) {
            Ok(_) => (),
            Err(e) => global_errors.push(e),
        }
        Self {
            db,
            main_file,
            entity_route_sheet: EntityRouteSheet::new(ast_text, global_errors),
            trait_uses: LocalStack::new(),
        }
    }

    pub(super) fn finish(self) -> EntityRouteSheet {
        self.entity_route_sheet
    }

    pub(super) fn infer_all(&mut self, ast_iter: AstIter) {
        self.enter_block();
        let arena = self.entity_route_sheet.ast_text.arena.clone();
        for item in ast_iter {
            if let Some(children) = item.opt_children {
                match item.value {
                    Ok(value) => match value.kind {
                        AstKind::TypeDefnHead { .. } | AstKind::EnumVariantDefnHead { .. } => {
                            self.infer_all(children)
                        }
                        AstKind::MainDefn => {
                            let opt_output_ty = self.db.global_output_ty(self.main_file).ok();
                            self.infer_morphism(&[], opt_output_ty, children, &arena)
                        }
                        AstKind::DatasetConfigDefnHead => self.infer_routine(
                            &[],
                            RootIdentifier::DatasetType.into(),
                            children,
                            &arena,
                        ),
                        AstKind::RoutineDefnHead(ref head) => self.infer_routine(
                            &head.input_placeholders,
                            head.output_ty.route,
                            children,
                            &arena,
                        ),
                        AstKind::TypeAssociatedRoutineDefnHead(ref head) => self.infer_routine(
                            &head.input_placeholders,
                            head.output_ty.route,
                            children,
                            &arena,
                        ),
                        AstKind::PatternDefnHead => todo!(),
                        AstKind::Use { .. } => (),
                        AstKind::FieldDefnHead(ref head) => match head.kind {
                            FieldKind::StructOriginal => (),
                            FieldKind::RecordOriginal => (),
                            FieldKind::StructDerived | FieldKind::RecordDerived => {
                                self.infer_morphism(&[], Some(head.ty), children, &arena)
                            }
                        },
                        AstKind::Stmt(_) => todo!(),
                        AstKind::TypeMethodDefnHead(ref head) => self.infer_routine(
                            &head.input_placeholders,
                            head.output_ty.route,
                            children,
                            &arena,
                        ),
                        AstKind::FeatureDecl { ty, .. } => {
                            self.infer_morphism(&[], Some(ty.route), children, &arena)
                        }
                        AstKind::Submodule { ident, source_file } => (),
                    },
                    _ => (),
                }
            }
        }
        self.exit_block()
    }

    fn add_inputs(&mut self, inputs: &[InputPlaceholder]) {
        if inputs.len() > 0 {
            if let None = self
                .entity_route_sheet
                .variable_tys
                .get(&(inputs[0].ident.ident, inputs[0].ranged_ty.row()))
            {
                for input in inputs {
                    should!(self
                        .entity_route_sheet
                        .variable_tys
                        .insert(
                            (input.ident.ident, inputs[0].ranged_ty.row()),
                            input.ranged_ty.route,
                        )
                        .is_none());
                }
            }
        }
    }
}
