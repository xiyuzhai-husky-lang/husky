mod impl_expr;
mod impl_function;
mod impl_locality;
mod impl_stmt;

use super::*;
use fold::LocalStack;
use husky_ast::{AstIter, AstText};
use husky_text::TextRanged;
use std::sync::Arc;

pub struct EntityRouteSheetBuilder<'a> {
    db: &'a dyn InferEntityRouteQueryGroup,
    arena: &'a RawExprArena,
    main_file: FilePtr,
    entity_route_sheet: EntityRouteSheet,
    trait_uses: LocalStack<EntityRouteVariant>,
}

impl<'a> EntityRouteSheetBuilder<'a> {
    fn file(&self) -> FilePtr {
        self.entity_route_sheet.ast_text.file
    }

    pub(super) fn new(
        db: &'a dyn InferEntityRouteQueryGroup,
        arena: &'a RawExprArena,
        ast_text: Arc<AstText>,
    ) -> Self {
        let main_file = db.main_file(ast_text.file).unwrap();
        let mut global_errors = Vec::new();
        match db.crate_input_ty(main_file) {
            Ok(_) => (),
            Err(e) => global_errors.push(e),
        }
        match db.crate_output_ty(main_file) {
            Ok(_) => (),
            Err(e) => global_errors.push(e),
        }
        Self {
            db,
            arena,
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
            let ast = match item.value.as_ref() {
                Ok(ast) => ast,
                Err(_) => continue,
            };
            match ast.variant {
                AstVariant::FieldDefnHead {
                    liason,
                    ranged_ident,
                    field_ty: ty,
                    field_ast_kind: field_kind,
                } => match field_kind {
                    FieldAstKind::StructDefault { default } => {
                        self.infer_expr(default, Some(ty.route));
                    }
                    FieldAstKind::StructDerivedEager { derivation } => {
                        self.infer_expr(derivation, Some(ty.route));
                    }
                    _ => (),
                },
                _ => (),
            }
            if let Some(children) = item.opt_children {
                match ast.variant {
                    AstVariant::TypeDefnHead { .. } | AstVariant::EnumVariantDefnHead { .. } => {
                        self.infer_all(children)
                    }
                    AstVariant::MainDefnHead => {
                        let opt_output_ty = self.db.crate_output_ty(self.main_file).ok();
                        self.infer_function(&[], opt_output_ty, children)
                    }
                    AstVariant::DatasetConfigDefnHead => {
                        self.infer_function(&[], Some(RootIdentifier::DatasetType.into()), children)
                    }
                    AstVariant::CallFormDefnHead {
                        ref parameters,
                        output_ty,
                        ..
                    } => self.infer_function(parameters, Some(output_ty.route), children),
                    AstVariant::Visual => self.infer_function(&[], None, children),
                    AstVariant::Use { .. } => (),
                    AstVariant::FieldDefnHead {
                        field_ast_kind: field_kind,
                        field_ty: ty,
                        ..
                    } => match field_kind {
                        FieldAstKind::StructOriginal => (),
                        FieldAstKind::RecordOriginal => (),
                        FieldAstKind::StructDerivedLazy { .. } | FieldAstKind::RecordDerived => {
                            self.infer_function(&[], Some(ty.route), children)
                        }
                        FieldAstKind::StructDefault { .. } => todo!(),
                        FieldAstKind::StructDerivedEager { .. } => todo!(),
                    },
                    AstVariant::Stmt(_) => todo!(),
                    AstVariant::FeatureDefnHead { output_ty: ty, .. } => {
                        self.infer_function(&[], Some(ty.route), children)
                    }
                    AstVariant::Submodule { ident, source_file } => (),
                }
            }
        }
        self.exit_block()
    }

    fn add_inputs(&mut self, inputs: &[Parameter]) {
        for input in inputs {
            should!(self
                .entity_route_sheet
                .variable_tys
                .insert(
                    (input.ranged_ident.ident, input.ranged_ident.range),
                    input.ranged_ty.route,
                )
                .is_none());
        }
    }
}
