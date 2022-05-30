mod eager;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstKind};
use defn_head::FieldKind;
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
                        AstKind::TypeDefnHead { .. } | AstKind::EnumVariantDefnHead { .. } => {
                            self.infer_all(children)
                        }
                        AstKind::MainDefn => self.infer_lazy_stmts(children, &arena),
                        AstKind::DatasetConfigDefnHead => self.infer_eager_stmts(children, &arena),
                        AstKind::CallFormDefnHead(ref head) => {
                            self.infer_eager_stmts(children, &arena)
                        }
                        AstKind::CallFormDefnHead(ref head) => {
                            self.infer_eager_stmts(children, &arena)
                        }
                        AstKind::Visual => self.infer_eager_stmts(children, &arena),
                        AstKind::PatternDefnHead => todo!(),
                        AstKind::Use { .. } => (),
                        AstKind::FieldDefnHead { ref head, .. } => match head.field_kind {
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
                                p!(head.field_kind);
                                todo!()
                            }
                        },
                        AstKind::Stmt(_) => todo!(),
                        AstKind::CallFormDefnHead(ref head) => {
                            self.infer_eager_stmts(children, &arena)
                        }
                        AstKind::FeatureDecl { ty, .. } => self.infer_lazy_stmts(children, &arena),
                        AstKind::Submodule { ident, source_file } => (),
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
