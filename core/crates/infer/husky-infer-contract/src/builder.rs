mod eager;
mod lazy;

use husky_ast::{AstIter, AstVariant, FieldAstKind, RawExpr, RawExprArena};
use husky_entity_route::EntityRoutePtr;
use husky_entity_syntax::EntitySyntaxResult;
use husky_infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use husky_word::{Paradigm, RootIdentifier};
use infer_decl::DeclQueryGroup;

use crate::*;

pub struct ContractSheetBuilder<'a> {
    db: &'a dyn InferContractSalsaQueryGroup,
    arena: &'a RawExprArena,
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
        arena: &'a RawExprArena,
        file: FilePtr,
    ) -> EntitySyntaxResult<Self> {
        Ok(Self {
            db,
            arena,
            contract_sheet: ContractSheet::new(db.entity_route_sheet(file)?),
        })
    }

    pub(crate) fn infer_all(&mut self, ast_iter: AstIter) {
        for item in ast_iter {
            let ast = match item.value.as_ref() {
                Ok(ast) => ast,
                Err(_) => continue,
            };
            match ast.variant {
                AstVariant::FieldDefnHead {
                    liason,
                    field_ty: ty,
                    field_ast_kind,
                    ..
                } => match field_ast_kind {
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
                            self.infer_eager_expr(default, contract)
                        }
                    }
                    FieldAstKind::StructDerivedEager { derivation } => {
                        msg_once!("todo: handle ref");
                        if let Ok(is_field_copyable) = self.db.is_copyable(ty.route) {
                            let contract = match is_field_copyable {
                                true => EagerContract::Pure,
                                false => EagerContract::Move,
                            };
                            self.infer_eager_expr(derivation, contract)
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
                    AstVariant::MainDefnHead => self.infer_lazy_stmts(children),
                    AstVariant::DatasetConfigDefnHead => {
                        self.infer_eager_stmts(children, RootIdentifier::DatasetType.into())
                    }
                    AstVariant::CallFormDefnHead { return_ty, .. } => {
                        self.infer_eager_stmts(children, return_ty.route)
                    }
                    AstVariant::Visual => self.infer_lazy_stmts(children),
                    AstVariant::Use { .. } => (),
                    AstVariant::FieldDefnHead {
                        field_ast_kind,
                        field_ty: ty,
                        ..
                    } => match field_ast_kind {
                        FieldAstKind::StructDerivedLazy {
                            paradigm: Paradigm::EagerProcedural | Paradigm::EagerFunctional,
                        } => self.infer_eager_stmts(children, ty.route),
                        FieldAstKind::StructDerivedLazy {
                            paradigm: Paradigm::LazyFunctional,
                        }
                        | FieldAstKind::RecordDerived => self.infer_lazy_stmts(children),
                        _ => (),
                    },
                    AstVariant::FeatureDefnHead {
                        paradigm,
                        return_ty: ty,
                        ..
                    } => match paradigm {
                        Paradigm::LazyFunctional => self.infer_lazy_stmts(children),
                        Paradigm::EagerFunctional | Paradigm::EagerProcedural => {
                            self.infer_eager_stmts(children, ty.route)
                        }
                    },
                    AstVariant::Submodule { .. } => (),
                    AstVariant::Stmt(_) => panic!(),
                }
            }
        }
    }

    pub(crate) fn finish(self) -> ContractSheet {
        self.contract_sheet
    }
}
