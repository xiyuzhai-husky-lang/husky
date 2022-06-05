mod eager;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstVariant, FieldAstKind};
use entity_route::{EntityRouteKind, EntityRoutePtr};
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
                        msg_once!("todo: handle ref");
                        if let Ok(is_field_copyable) = self.db.is_copyable(ty.route) {
                            let contract = match is_field_copyable {
                                true => EagerContract::Pure,
                                false => match liason {
                                    MemberLiason::Immutable | MemberLiason::Mutable => {
                                        EagerContract::Move
                                    }
                                    MemberLiason::Derived => panic!(),
                                },
                            };
                            self.infer_eager_expr(default, contract, &arena)
                        }
                    }
                    FieldAstKind::StructDerivedEager { derivation } => {
                        msg_once!("todo: handle ref");
                        if let Ok(is_field_copyable) = self.db.is_copyable(ty.route) {
                            let contract = match is_field_copyable {
                                true => EagerContract::Pure,
                                false => EagerContract::Move,
                            };
                            self.infer_eager_expr(derivation, contract, &arena)
                        }
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
                    AstVariant::MainDefn => self.infer_lazy_stmts(children, &arena),
                    AstVariant::DatasetConfigDefnHead => {
                        self.infer_eager_stmts(children, &arena, RootIdentifier::DatasetType.into())
                    }
                    AstVariant::CallFormDefnHead { output_ty, .. } => {
                        self.infer_eager_stmts(children, &arena, output_ty.route)
                    }
                    AstVariant::Visual => self.infer_eager_stmts(
                        children,
                        &arena,
                        EntityRoutePtr::Root(RootIdentifier::VisualType),
                    ),
                    AstVariant::Use { .. } => (),
                    AstVariant::FieldDefnHead {
                        field_ast_kind: field_kind,
                        liason,
                        ranged_ident,
                        ty,
                    } => match field_kind {
                        FieldAstKind::StructDerivedLazy {
                            paradigm: Paradigm::EagerProcedural | Paradigm::EagerFunctional,
                        } => self.infer_eager_stmts(children, &arena, ty.route),
                        FieldAstKind::StructDerivedLazy {
                            paradigm: Paradigm::LazyFunctional,
                        }
                        | FieldAstKind::RecordDerived => self.infer_lazy_stmts(children, &arena),
                        _ => (),
                    },
                    AstVariant::Stmt(_) => todo!(),
                    AstVariant::FeatureDecl { ty, .. } => self.infer_lazy_stmts(children, &arena),
                    AstVariant::Submodule { ident, source_file } => (),
                }
            }
        }
    }

    pub(crate) fn finish(self) -> ContractSheet {
        self.contract_sheet
    }
}
