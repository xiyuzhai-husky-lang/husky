mod eager;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstKind, RawExprArena};
use entity_kind::FieldKind;
use entity_route_query::EntitySyntaxResult;
use file::FilePtr;
use infer_contract::{ContractSheet, InferContract};
use infer_decl::DeclQueryGroup;
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use word::RootIdentifier;

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
        use fold::FoldStorage;
        let arena = self.entity_route_sheet.ast_text.arena.clone();
        let ast_text = self.entity_route_sheet.ast_text.clone();
        self.infer_ast(ast_text.folded_results.iter(), &arena)
    }

    pub(super) fn infer_ast(&mut self, ast_iter: AstIter, arena: &RawExprArena) {
        for item in ast_iter {
            if let Some(children) = item.opt_children {
                match item.value {
                    Ok(value) => match value.kind {
                        AstKind::TypeDefnHead { .. } | AstKind::EnumVariantDefnHead { .. } => {
                            self.infer_ast(children, arena)
                        }
                        AstKind::MainDefn => self.infer_morphism(
                            &arena,
                            &[],
                            children,
                            self.db.global_output_ty(self.main_file).ok(),
                            OutputLiason::Transfer,
                        ),
                        AstKind::DatasetConfigDefnHead => self.infer_routine(
                            &arena,
                            &[],
                            children,
                            Some(EntityRoutePtr::Root(RootIdentifier::DatasetType)),
                            OutputLiason::Transfer,
                        ),
                        AstKind::RoutineDefnHead(ref head) => self.infer_routine(
                            &arena,
                            &head.input_placeholders,
                            children,
                            Some(head.output_ty.route),
                            head.output_contract,
                        ),
                        AstKind::TypeAssociatedRoutineDefnHead(ref head) => self.infer_routine(
                            &arena,
                            &head.input_placeholders,
                            children,
                            Some(head.output_ty.route),
                            head.output_contract,
                        ),
                        AstKind::PatternDefnHead => todo!(),
                        AstKind::Use { .. } => (),
                        AstKind::FieldDefnHead(ref head) => match head.kind {
                            FieldKind::StructOriginal => (),
                            FieldKind::RecordOriginal => (),
                            FieldKind::StructDerived | FieldKind::RecordDerived => self
                                .infer_morphism(
                                    &arena,
                                    &[],
                                    children,
                                    Some(head.ty),
                                    OutputLiason::Transfer,
                                ),
                        },
                        AstKind::Stmt(_) => todo!(),
                        AstKind::TypeMethodDefnHead(ref head) => self.infer_routine(
                            &arena,
                            &head.input_placeholders,
                            children,
                            Some(head.output_ty.route),
                            head.output_contract,
                        ),
                        AstKind::FeatureDecl { ty, .. } => self.infer_morphism(
                            &arena,
                            &[],
                            children,
                            Some(ty.route),
                            OutputLiason::Transfer,
                        ),
                        AstKind::Submodule { ident, source_file } => (),
                    },
                    _ => (),
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
