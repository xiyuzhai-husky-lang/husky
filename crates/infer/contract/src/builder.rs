mod eager;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstVariant, FieldKind};
use entity_route::EntityRouteKind;
use entity_syntax::EntitySyntaxResult;
use fold::LocalStack;
use infer_decl::DeclQueryGroup;
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use word::{Paradigm, RootIdentifier};

use crate::*;

pub struct ContractSheetBuilder<'a> {
    db: &'a dyn InferContractSalsaQueryGroup,
    file: FilePtr,
    main_file: FilePtr,
    contract_sheet: ContractSheet,
}

impl<'a> InferEntityRoute for ContractSheetBuilder<'a> {
    fn decl_db(&self) -> &dyn DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &EntityRouteSheet {
        &self.contract_sheet.entity_route_sheet
    }
}

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn new(
        db: &'a dyn InferContractSalsaQueryGroup,
        file: FilePtr,
    ) -> EntitySyntaxResult<Self> {
        Ok(Self {
            db,
            file,
            main_file: db.main_file(file).unwrap(),
            contract_sheet: ContractSheet::new(db.entity_route_sheet(file)?),
        })
    }

    pub(crate) fn infer_all(&mut self, ast_iter: AstIter) {
        let arena = self
            .contract_sheet
            .entity_route_sheet
            .ast_text
            .arena
            .clone();
        for item in ast_iter {
            if let Some(children) = item.opt_children {
                match item.value {
                    Ok(value) => match value.variant {
                        AstVariant::TypeDefnHead { .. }
                        | AstVariant::EnumVariantDefnHead { .. } => self.infer_all(children),
                        AstVariant::MainDefn => self.infer_lazy_stmts(children, &arena),
                        AstVariant::DatasetConfigDefnHead => {
                            self.infer_eager_stmts(children, &arena)
                        }
                        AstVariant::CallFormDefnHead(ref head) => {
                            self.infer_eager_stmts(children, &arena)
                        }
                        AstVariant::CallFormDefnHead(ref head) => {
                            self.infer_eager_stmts(children, &arena)
                        }
                        AstVariant::Visual => self.infer_eager_stmts(children, &arena),
                        AstVariant::PatternDefnHead => todo!(),
                        AstVariant::Use { .. } => (),
                        AstVariant::FieldDefnHead { field_kind, .. } => match field_kind {
                            FieldKind::StructOriginal => (),
                            FieldKind::RecordOriginal => (),
                            FieldKind::StructDerivedLazy {
                                paradigm: Paradigm::EagerProcedural | Paradigm::EagerFunctional,
                            } => self.infer_eager_stmts(children, &arena),
                            FieldKind::StructDerivedLazy {
                                paradigm: Paradigm::LazyFunctional,
                            }
                            | FieldKind::RecordDerived => self.infer_lazy_stmts(children, &arena),
                            _ => {
                                p!(field_kind);
                                todo!()
                            }
                        },
                        AstVariant::Stmt(_) => todo!(),
                        AstVariant::CallFormDefnHead(ref head) => {
                            self.infer_eager_stmts(children, &arena)
                        }
                        AstVariant::FeatureDecl { ty, .. } => {
                            self.infer_lazy_stmts(children, &arena)
                        }
                        AstVariant::Submodule { ident, source_file } => (),
                    },
                    _ => (),
                }
            }
        }
    }

    pub(crate) fn finish(self) -> ContractSheet {
        self.contract_sheet
    }
}
