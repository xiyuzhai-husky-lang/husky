use std::ops::AddAssign;

use ast::{Ast, AstContext, AstKind, AstResult, RawExpr, RawExprVariant, RawStmtKind};
use defn_head::InputPlaceholder;
use entity_kind::{RoutineContextKind, TyKind};
use entity_route::EntityRoutePtr;
use fold::LocalValue;
use syntax_types::*;
use vm::{FieldContract, InitKind, InputContract, PrimitiveValue};
use word::{RootIdentifier, WordAllocator};

pub struct Formatter<'a> {
    word_unique_allocator: &'a WordAllocator,
    arena: &'a ast::RawExprArena,
    indent: fold::Indent,
    result: String,
    context: LocalValue<AstContext>,
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(
        word_unique_allocator: &'a WordAllocator,
        arena: &'a ast::RawExprArena,
        context: AstContext,
    ) -> Self {
        Self {
            word_unique_allocator,
            arena,
            indent: 0,
            result: String::new(),
            context: LocalValue::new(context),
        }
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}

impl<'a> fold::Executor<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>> for Formatter<'a> {
    fn _enter_block(&mut self) {
        self.context.enter()
    }

    fn _exit_block(&mut self) {
        self.context.exit()
    }

    fn execute(
        &mut self,
        indent: fold::Indent,
        ast_result: &AstResult<Ast>,
        enter_block: impl FnOnce(&mut Self),
    ) {
        self.indent = indent;
        if self.result.len() > 0 {
            self.newline();
        }
        self.fmt(ast_result.as_ref().unwrap(), enter_block);
    }
}

impl<'a> Formatter<'a> {
    fn newline(&mut self) {
        self.result
            .reserve(self.result.len() + self.indent as usize + 1);
        self.result.push('\n');
        for _ in 0..self.indent {
            self.result.push(' ');
        }
    }

    fn write(&mut self, s: &str) {
        self.result += s;
    }
}

impl<'a> Formatter<'a> {
    fn fmt(&mut self, ast: &ast::Ast, enter_block: impl FnOnce(&mut Self)) {
        match ast.kind {
            AstKind::TypeDefnHead {
                ident,
                ref kind,
                generic_placeholders: ref generics,
            } => {
                enter_block(self);
                match kind {
                    TyKind::Enum => todo!(),
                    TyKind::Struct => {
                        self.context.set_value(AstContext::Struct);
                        self.write("struct ")
                    }
                    TyKind::Record => todo!(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Other => todo!(),
                }
                self.fmt_ident(ident.ident.into());
                if generics.len() > 0 {
                    todo!()
                }
            }
            AstKind::MainDefn => {
                enter_block(self);
                self.context.set_value(AstContext::Main);
                self.write("main:")
            }
            AstKind::RoutineDefnHead(ref head) => {
                enter_block(self);
                self.context.set_value((head.routine_kind).into());
                self.write(match head.routine_kind {
                    RoutineContextKind::Test => "test ",
                    RoutineContextKind::Proc => "proc ",
                    RoutineContextKind::Func => "func ",
                });
                self.write(&head.ident.ident);
                self.write("(");
                for i in 0..head.input_placeholders.len() {
                    if i > 0 {
                        self.write(", ");
                    }
                    let input_placeholder = &head.input_placeholders[i];
                    self.fmt_ident(input_placeholder.ident.ident.into());
                    self.write(": ");
                    self.fmt_func_input_contracted_type(input_placeholder);
                }
                self.write(")");
                if head.output_ty.route != EntityRoutePtr::Root(RootIdentifier::Void) {
                    self.write(" -> ");
                    self.fmt_ty(head.output_ty.route);
                }
                self.write(":");
            }
            AstKind::PatternDefnHead => todo!(),
            AstKind::FieldDefnHead(ref field_var) => {
                self.fmt_ident(field_var.ident.ident.into());
                self.write(": ");
                self.fmt_member_variable_contracted_type(field_var.contract, field_var.ty)
            }
            AstKind::Stmt(ref stmt) => self.fmt_stmt(stmt),
            AstKind::DatasetConfigDefnHead => todo!(),
            AstKind::EnumVariantDefnHead {
                ident,
                variant_class: ref variant_kind,
            } => todo!(),
            AstKind::TypeMethodDefnHead { .. } => todo!(),
            AstKind::FeatureDecl { .. } => todo!(),
            AstKind::Use { ref use_variant } => todo!(),
        }
    }

    fn fmt_ident(&mut self, ident: word::Identifier) {
        self.result.add_assign(&ident)
    }

    fn fmt_member_variable_contracted_type(&mut self, contract: FieldContract, ty: EntityRoutePtr) {
        match contract {
            FieldContract::Own => (),
            FieldContract::GlobalRef => self.write("&"),
            FieldContract::LazyOwn => todo!(),
        }
        self.fmt_ty(ty);
    }

    fn fmt_func_input_contracted_type(&mut self, ty: &InputPlaceholder) {
        match ty.contract {
            InputContract::Pure => (),
            InputContract::GlobalRef => self.write("&"),
            InputContract::Move => self.write("!"),
            InputContract::BorrowMut => self.write("mut &"),
            InputContract::MoveMut => self.write("mut !"),
            InputContract::Exec => todo!(),
            InputContract::MemberAccess => todo!(),
        }
        self.fmt_ty(ty.ranged_ty.route);
    }

    fn fmt_ty(&mut self, ty: EntityRoutePtr) {
        match ty {
            EntityRoutePtr::Root(ident) => self.write(&ident),
            EntityRoutePtr::Custom(_) => todo!(),
            EntityRoutePtr::ThisType => self.write("This"),
        }
    }

    fn fmt_stmt(&mut self, stmt: &ast::RawStmt) {
        match stmt.kind {
            RawStmtKind::Loop(_) => todo!(),
            RawStmtKind::Branch(_) => todo!(),
            RawStmtKind::Exec(expr) => self.fmt_expr(&self.arena[expr]),
            RawStmtKind::Init {
                init_kind: kind,
                varname,
                initial_value,
            } => {
                match kind {
                    InitKind::Let => self.write("let "),
                    InitKind::Var => self.write("var "),
                    InitKind::Decl => (),
                }
                self.fmt_ident(varname.ident.into());
                self.write(" = ");
                self.fmt_expr(&self.arena[initial_value]);
            }
            RawStmtKind::Return(expr) => {
                match self.context.value() {
                    AstContext::Func | AstContext::Morphism | AstContext::Main => (),
                    AstContext::Proc => self.write("return "),
                    AstContext::Package(_) => todo!(),
                    AstContext::Module(_) => todo!(),
                    AstContext::DatasetConfig => todo!(),
                    AstContext::Test => todo!(),
                    AstContext::Struct => todo!(),
                    AstContext::Record => todo!(),
                    AstContext::Props => todo!(),
                    AstContext::Enum => todo!(),
                }
                self.fmt_expr(&self.arena[expr]);
            }
            RawStmtKind::Assert(expr) => {
                self.write("assert ");
                self.fmt_expr(&self.arena[expr]);
            }
        }
    }

    fn fmt_expr(&mut self, expr: &RawExpr) {
        match expr.variant {
            RawExprVariant::Variable { varname, .. } => self.write(&varname),
            RawExprVariant::Unrecognized(varname) => self.write(&varname),
            RawExprVariant::PrimitiveLiteral(literal) => match literal {
                PrimitiveValue::I32(i) => self.write(&i.to_string()),
                PrimitiveValue::F32(f) => self.write(&f.to_string()),
                PrimitiveValue::Void => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::Bool(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
            },
            RawExprVariant::Bracketed(expr_idx) => {
                self.write("(");
                self.fmt_expr(&self.arena[expr_idx]);
                self.write(")");
            }
            RawExprVariant::Opn { opr: opn, ref opds } => match opn {
                Opr::Binary(opr) => {
                    let opds = &self.arena[opds];
                    self.fmt_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.fmt_expr(&opds[1]);
                }
                Opr::Prefix(opr) => todo!(),
                Opr::Suffix(_) => todo!(),
                Opr::List(opr) => match opr {
                    ListOpr::TupleInit => todo!(),
                    ListOpr::NewVec => todo!(),
                    ListOpr::NewDict => todo!(),
                    ListOpr::Call => todo!(),
                    ListOpr::Index => todo!(),
                    ListOpr::ModuloIndex => todo!(),
                    ListOpr::StructInit => todo!(),
                },
                // ast::Opr::ScopeCall(_) => todo!(),
                // ast::Opr::ValueCall => {
                //     let opds = &self.arena[opds];
                //     self.fmt_expr(result, &opds[0]);
                //     self.write("(");
                //     for i in 1..opds.len() {
                //         if i >= 2 {
                //             self.write(", ")
                //         }
                //         self.fmt_expr(result, &opds[i]);
                //     }
                //     self.write(")");
                // }
                // ast::Opr::MemberCall(_) => todo!(),
                // ast::Opr::Member(_) => todo!(),
                // ast::Opr::Index => todo!(),
                // ast::Opr::Opr(opr) => match opr {},
            },
            RawExprVariant::Entity { .. } => todo!(),
            RawExprVariant::Lambda(ref inputs, expr) => {
                self.write("|");
                self.join(
                    inputs,
                    |this, (ident, ty)| {
                        this.fmt_ident((*ident).ident.into());
                        if let Some(ty) = ty {
                            this.write(": ");
                            this.fmt_ty(ty.route)
                        }
                    },
                    ", ",
                );
                self.write("| ");
                self.fmt_expr(&self.arena[expr])
            }
            RawExprVariant::This { .. } => todo!(),
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
        }
    }

    fn join<T>(&mut self, items: &[T], f: fn(&mut Self, item: &T), separator: &'static str) {
        for i in 0..items.len() {
            if i > 0 {
                self.write(separator);
            }
            f(self, &items[i]);
        }
    }
}
