mod eager;
mod lazy;

use std::sync::Arc;

use ast::*;
use entity_syntax::EntitySyntaxResult;
use file::FilePtr;
use infer_contract::{ContractSheet, InferContract};
use infer_decl::DeclQueryGroup;
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use word::{Paradigm, RootIdentifier};

use crate::*;

pub(crate) struct QualifiedTySheetBuilder<'a> {
    db: &'a dyn InferQualifiedTyQueryGroup,
    contract_sheet: Arc<ContractSheet>,
    entity_route_sheet: Arc<EntityRouteSheet>,
    qualified_ty_sheet: QualifiedTySheet,
    main_file: FilePtr,
}

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn new(
        db: &'a dyn InferQualifiedTyQueryGroup,
        file: FilePtr,
    ) -> EntitySyntaxResult<Self> {
        let contract_sheet = db.contract_sheet(file)?;
        Ok(Self {
            db,
            entity_route_sheet: contract_sheet.entity_route_sheet.clone(),
            qualified_ty_sheet: QualifiedTySheet::new(contract_sheet.clone()),
            contract_sheet,
            main_file: db.main_file(file).unwrap(),
        })
    }

    pub(super) fn infer_all(&mut self) {
        use fold::FoldableStorage;
        let arena = self.entity_route_sheet.ast_text.arena.clone();
        let ast_text = self.entity_route_sheet.ast_text.clone();
        self.infer_ast(ast_text.folded_results.iter(), &arena)
    }

    pub(super) fn infer_ast(&mut self, ast_iter: AstIter, arena: &RawExprArena) {
        for item in ast_iter {
            let ast = match item.value.as_ref() {
                Ok(ast) => ast,
                Err(_) => continue,
            };
            match ast.variant {
                AstVariant::FieldDefnHead {
                    liason,
                    ranged_ident,
                    ty,
                    field_ast_kind: field_kind,
                } => match field_kind {
                    FieldAstKind::StructDefault { default } => {
                        self.infer_eager_expr(arena, default);
                    }
                    FieldAstKind::StructDerivedEager { derivation } => {
                        self.infer_eager_expr(arena, derivation);
                    }
                    _ => (),
                },
                _ => (),
            }
            if let Some(children) = item.opt_children {
                match ast.variant {
                    AstVariant::TypeDefnHead { .. } | AstVariant::EnumVariantDefnHead { .. } => {
                        self.infer_ast(children, arena)
                    }
                    AstVariant::MainDefn => self.infer_lazy_call_form(
                        &arena,
                        &[],
                        children,
                        self.db.global_output_ty(self.main_file).ok(),
                        OutputLiason::Transfer,
                    ),
                    AstVariant::DatasetConfigDefnHead => self.infer_eager_call_form(
                        &arena,
                        &[],
                        children,
                        Some(EntityRoutePtr::Root(RootIdentifier::DatasetType)),
                        OutputLiason::Transfer,
                    ),
                    AstVariant::CallFormDefnHead {
                        ref parameters,
                        output_ty,
                        output_liason,
                        ..
                    } => self.infer_eager_call_form(
                        &arena,
                        parameters,
                        children,
                        Some(output_ty.route),
                        output_liason,
                    ),
                    AstVariant::Visual => self.infer_lazy_call_form(
                        &arena,
                        &[],
                        children,
                        None,
                        OutputLiason::Transfer,
                    ),
                    AstVariant::Use { .. } => (),
                    AstVariant::FieldDefnHead {
                        field_ast_kind: field_kind,
                        ty,
                        ..
                    } => match field_kind {
                        FieldAstKind::StructOriginal => (),
                        FieldAstKind::RecordOriginal => (),
                        FieldAstKind::StructDerivedLazy {
                            paradigm: Paradigm::EagerProcedural | Paradigm::EagerFunctional,
                        } => self.infer_eager_call_form(
                            &arena,
                            &[],
                            children,
                            Some(ty.route),
                            OutputLiason::Transfer,
                        ),
                        FieldAstKind::StructDerivedLazy {
                            paradigm: Paradigm::LazyFunctional,
                        }
                        | FieldAstKind::RecordDerived => self.infer_lazy_call_form(
                            &arena,
                            &[],
                            children,
                            Some(ty.route),
                            OutputLiason::Transfer,
                        ),
                        _ => todo!(),
                    },
                    AstVariant::Stmt(_) => (),
                    AstVariant::FeatureDecl { ty, .. } => self.infer_lazy_call_form(
                        &arena,
                        &[],
                        children,
                        Some(ty.route),
                        OutputLiason::Transfer,
                    ),
                    AstVariant::Submodule { ident, source_file } => (),
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

    fn entity_route_sheet(&self) -> &infer_entity_route::EntityRouteSheet {
        &self.entity_route_sheet
    }
}
