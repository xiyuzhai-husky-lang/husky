use crate::*;
use ast::AstIter;

pub struct TySheetBuilder<'a> {
    db: &'a dyn InferSalsaQueryGroup,
    ty_sheet: TySheet,
}

impl<'a> TySheetBuilder<'a> {
    pub(super) fn new(db: &'a dyn InferSalsaQueryGroup) -> Self {
        Self {
            db,
            ty_sheet: Default::default(),
        }
    }

    pub(super) fn finish(self) -> TySheet {
        self.ty_sheet
    }

    pub(super) fn infer_all(&mut self, ast_iter: AstIter) {
        for item in ast_iter {
            match item.value {
                Ok(value) => match value.kind {
                    AstKind::TypeDef { .. } => self.infer_all(item.children.unwrap()),
                    AstKind::MainDecl => todo!(),
                    AstKind::DatasetConfig => todo!(),
                    AstKind::RoutineDecl {
                        ref routine_head, ..
                    } => self.infer_routine(item.children.unwrap()),
                    AstKind::PatternDef => todo!(),
                    AstKind::Use { ident, scope } => todo!(),
                    AstKind::MembVar { .. } => (),
                    AstKind::MembRoutineDecl(_) => todo!(),
                    AstKind::Stmt(_) => todo!(),
                    AstKind::EnumVariant {
                        ident,
                        ref raw_variant_kind,
                    } => todo!(),
                },
                _ => (),
            }
        }
    }

    fn infer_routine(&mut self, ast_iter: AstIter) {
        todo!()
    }
}
