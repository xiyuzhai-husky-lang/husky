mod impl_expr;
mod impl_function;
mod impl_locality;
mod impl_stmt;

use super::*;
use fold::LocalStack;
use husky_ast::{AstIter, AstText};
use std::sync::Arc;

pub struct EntityRouteSheetBuilder<'a> {
    db: &'a dyn InferEntityRouteQueryGroup,
    arena: &'a RawExprArena,
    entity_route_sheet: EntityRouteSheet,
    trait_uses: LocalStack<EntityRouteVariant>,
}

impl<'a> EntityRouteSheetBuilder<'a> {
    fn file(&self) -> FileItd {
        self.entity_route_sheet.ast_text.file
    }

    pub(super) fn new(
        db: &'a dyn InferEntityRouteQueryGroup,
        arena: &'a RawExprArena,
        ast_text: Arc<AstText>,
    ) -> Self {
        let mut global_errors = Vec::new();
        match db.target_input_ty() {
            Ok(_) => (),
            Err(e) => global_errors.push(e),
        }
        match db.target_output_ty() {
            Ok(_) => (),
            Err(e) => global_errors.push(e),
        }
        Self {
            db,
            arena,
            entity_route_sheet: EntityRouteSheet::new(ast_text, global_errors),
            trait_uses: LocalStack::new(),
        }
    }

    pub(super) fn finish(self) -> EntityRouteSheet {
        self.entity_route_sheet
    }

    pub(super) fn infer_all(&mut self, ast_iter: AstIter) {
        self.enter_block();
        for item in ast_iter {
            let ast = match item.value.as_ref() {
                Ok(ast) => ast,
                Err(_) => continue,
            };
            match ast.variant {
                AstVariant::FieldDefnHead {
                    field_ty,
                    ast_field_kind,
                    ..
                } => match ast_field_kind {
                    AstFieldKind::StructDefault { default } => {
                        self.infer_expr(default, Some(field_ty.route));
                    }
                    AstFieldKind::StructDerivedEager { derivation } => {
                        self.infer_expr(derivation, Some(field_ty.route));
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
                        let opt_output_ty = self.db.target_output_ty().ok();
                        self.infer_function(&[], opt_output_ty, children)
                    }
                    AstVariant::DatasetConfigDefnHead => self.infer_function(
                        &[],
                        Some(RootBuiltinIdentifier::DatasetType.into()),
                        children,
                    ),
                    AstVariant::CallFormDefnHead {
                        ref parameters,
                        return_ty: output_ty,
                        ..
                    } => self.infer_function(parameters, Some(output_ty.route), children),
                    AstVariant::Visual => self.infer_function(&[], None, children),
                    AstVariant::Use { .. } => (),
                    AstVariant::FieldDefnHead {
                        ast_field_kind: field_kind,
                        field_ty: ty,
                        ..
                    } => match field_kind {
                        AstFieldKind::StructOriginal => (),
                        AstFieldKind::RecordOriginal => (),
                        AstFieldKind::StructProperty { .. } | AstFieldKind::RecordDerived => {
                            self.infer_function(&[], Some(ty.route), children)
                        }
                        AstFieldKind::StructDefault { .. } => todo!(),
                        AstFieldKind::StructDerivedEager { .. } => todo!(),
                    },
                    AstVariant::Stmt(_) => todo!(),
                    AstVariant::FeatureDefnHead { return_ty: ty, .. } => {
                        self.infer_function(&[], Some(ty.route), children)
                    }
                    AstVariant::Submodule { .. } => (),
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
                .insert((input.ident(), input.ranged_ident().range), input.ty(),)
                .is_none());
        }
    }
}
