mod eager;
mod lazy;

use std::sync::Arc;

use husky_ast::*;
use husky_entity_syntax::EntitySyntaxResult;
use husky_file::FilePtr;
use husky_infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use husky_word::{Paradigm, RootIdentifier};
use infer_contract::{ContractSheet, InferContract};
use infer_decl::DeclQueryGroup;

use crate::*;

pub(crate) struct QualifiedTySheetBuilder<'a> {
    db: &'a dyn InferQualifiedTyQueryGroup,
    arena: &'a RawExprArena,
    contract_sheet: Arc<ContractSheet>,
    entity_route_sheet: Arc<EntityRouteSheet>,
    qualified_ty_sheet: QualifiedTySheet,
}

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn new(
        db: &'a dyn InferQualifiedTyQueryGroup,
        arena: &'a RawExprArena,
        file: FilePtr,
    ) -> EntitySyntaxResult<Self> {
        let contract_sheet = db.contract_sheet(file)?;
        Ok(Self {
            db,
            arena,
            entity_route_sheet: contract_sheet.entity_route_sheet.clone(),
            qualified_ty_sheet: QualifiedTySheet::new(contract_sheet.clone()),
            contract_sheet,
        })
    }

    pub(super) fn infer_all(&mut self) {
        use fold::FoldableStorage;
        let ast_text = self.entity_route_sheet.ast_text.clone();
        self.infer_ast(ast_text.folded_results.iter())
    }

    pub(super) fn infer_ast(&mut self, ast_iter: AstIter) {
        for item in ast_iter {
            let ast = match item.value.as_ref() {
                Ok(ast) => ast,
                Err(_) => continue,
            };
            match ast.variant {
                AstVariant::FieldDefnHead { ast_field_kind, .. } => match ast_field_kind {
                    AstFieldKind::StructDefault { default } => {
                        self.insert_eager_expr_inference(default);
                    }
                    AstFieldKind::StructDerivedEager { derivation } => {
                        self.insert_eager_expr_inference(derivation);
                    }
                    _ => (),
                },
                _ => (),
            }
            if let Some(children) = item.opt_children {
                match ast.variant {
                    AstVariant::TypeDefnHead { .. } | AstVariant::EnumVariantDefnHead { .. } => {
                        self.infer_ast(children)
                    }
                    AstVariant::MainDefnHead => self.infer_lazy_call_form(
                        &[],
                        children,
                        self.db.target_output_ty().ok(),
                        OutputModifier::Transfer,
                    ),
                    AstVariant::DatasetConfigDefnHead => self.infer_eager_call_form(
                        &[],
                        children,
                        Some(EntityRoutePtr::Root(RootIdentifier::DatasetType)),
                        OutputModifier::Transfer,
                    ),
                    AstVariant::CallFormDefnHead {
                        ref parameters,
                        output_ty,
                        output_liason,
                        ..
                    } => self.infer_eager_call_form(
                        parameters,
                        children,
                        Some(output_ty.route),
                        output_liason,
                    ),
                    AstVariant::Visual => {
                        self.infer_lazy_call_form(&[], children, None, OutputModifier::Transfer)
                    }
                    AstVariant::Use { .. } => (),
                    AstVariant::FieldDefnHead {
                        ast_field_kind: field_kind,
                        field_ty: ty,
                        ..
                    } => match field_kind {
                        AstFieldKind::StructOriginal => (),
                        AstFieldKind::RecordOriginal => (),
                        AstFieldKind::StructProperty {
                            paradigm: Paradigm::EagerProcedural | Paradigm::EagerFunctional,
                        } => self.infer_eager_call_form(
                            &[],
                            children,
                            Some(ty.route),
                            OutputModifier::Transfer,
                        ),
                        AstFieldKind::StructProperty {
                            paradigm: Paradigm::LazyFunctional,
                        }
                        | AstFieldKind::RecordDerived => self.infer_lazy_call_form(
                            &[],
                            children,
                            Some(ty.route),
                            OutputModifier::Transfer,
                        ),
                        _ => todo!(),
                    },
                    AstVariant::Stmt(_) => (),
                    AstVariant::FeatureDefnHead {
                        paradigm,
                        output_ty: ty,
                        ..
                    } => match paradigm {
                        Paradigm::LazyFunctional => self.infer_lazy_call_form(
                            &[],
                            children,
                            Some(ty.route),
                            OutputModifier::Transfer,
                        ),
                        Paradigm::EagerFunctional | Paradigm::EagerProcedural => self
                            .infer_eager_call_form(
                                &[],
                                children,
                                Some(ty.route),
                                OutputModifier::Transfer,
                            ),
                    },
                    AstVariant::Submodule { .. } => (),
                }
            }
        }
    }

    pub(super) fn finish(self) -> Arc<QualifiedTySheet> {
        Arc::new(self.qualified_ty_sheet)
    }
}

impl<'a> InferContract for QualifiedTySheetBuilder<'a> {
    fn contract_sheet(&self) -> &ContractSheet {
        &self.contract_sheet
    }
}

impl<'a> InferEntityRoute for QualifiedTySheetBuilder<'a> {
    fn decl_db(&self) -> &dyn DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &husky_infer_entity_route::EntityRouteSheet {
        &self.entity_route_sheet
    }
}
