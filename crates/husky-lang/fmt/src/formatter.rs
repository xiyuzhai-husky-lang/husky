use std::ops::AddAssign;

use common::*;

use ast::{Ast, AstResult, RawExpr, RawExprKind, RawStmtKind};
use scope::ScopePtr;
use syntax_types::*;
use vm::{InputContract, PrimitiveValue};
use word::{BuiltinIdentifier, WordInterner};

pub struct Formatter<'a> {
    word_unique_allocator: &'a WordInterner,
    arena: &'a ast::RawExprArena,
    indent: fold::Indent,
    result: String,
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(
        word_unique_allocator: &'a WordInterner,
        arena: &'a ast::RawExprArena,
    ) -> Self {
        Self {
            word_unique_allocator,
            arena,
            indent: 0,
            result: String::new(),
        }
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}

impl<'a> fold::Executor<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>> for Formatter<'a> {
    fn _enter_block(&mut self) {}

    fn _exit_block(&mut self) {}

    fn execute(
        &mut self,
        indent: fold::Indent,
        input: &AstResult<Ast>,
        enter_block: impl FnOnce(&mut Self),
    ) {
        self.indent = indent;
        if self.result.len() > 0 {
            self.newline();
        }
        self.fmt(input.as_ref().unwrap());
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
    fn fmt(&mut self, ast: &ast::Ast) {
        match ast {
            ast::Ast::TypeDef {
                ident,
                kind,
                generics,
            } => {
                epin!();
                match kind {
                    TyKind::Enum(_) => todo!(),
                    TyKind::Struct => self.write("struct "),
                }
                self.fmt_ident((*ident).into());
                if generics.len() > 0 {
                    todo!()
                }
            }
            ast::Ast::MainDef => self.write("main:"),
            ast::Ast::FuncDef { kind, decl } => {
                self.write(match kind {
                    FuncKind::Test => "test ",
                    FuncKind::Proc => todo!(),
                    FuncKind::Func => "func ",
                    FuncKind::Def => todo!(),
                });
                self.write(&decl.funcname);
                self.write("(");
                for i in 0..decl.inputs.len() {
                    if i > 0 {
                        self.write(", ");
                    }
                    let (ident, ref ty) = decl.inputs[i];
                    self.fmt_ident(ident.into());
                    self.write(": ");
                    self.fmt_func_input_contracted_type(ty);
                }
                self.write(")");
                if decl.output != ScopePtr::Builtin(BuiltinIdentifier::Void) {
                    self.write(" -> ");
                    self.fmt_type(decl.output);
                }
                self.write(":");
            }
            ast::Ast::PatternDef => todo!(),
            ast::Ast::Use { ident, scope } => todo!(),
            ast::Ast::MembDef { ident, kind } => match kind {
                MembKind::MembVar { ty } => {
                    self.fmt_ident((*ident).into());
                    self.write(": ");
                    self.fmt_member_variable_contracted_type(ty);
                }
                MembKind::MembFunc {
                    this,
                    inputs,
                    output,
                    args,
                } => todo!(),
            },
            ast::Ast::Stmt(stmt) => self.fmt_stmt(stmt),
            ast::Ast::DatasetConfig => todo!(),
        }
    }

    fn fmt_ident(&mut self, ident: word::Identifier) {
        self.result.add_assign(&ident)
    }

    fn fmt_member_variable_contracted_type(&mut self, ty: &MembType) {
        match ty.contract {
            InputContract::Intact => todo!(),
            InputContract::Share => todo!(),
            InputContract::Own => (),
        }
        self.fmt_type(ty.scope);
    }

    fn fmt_func_input_contracted_type(&mut self, ty: &InputType) {
        match ty.contract {
            InputContract::Intact => (),
            InputContract::Share => self.write("&"),
            InputContract::Own => self.write("!"),
        }
        self.fmt_type(ty.ty);
    }

    fn fmt_type(&mut self, ty: ScopePtr) {
        match ty {
            ScopePtr::Builtin(ident) => self.write(&ident),
            ScopePtr::Custom(_) => todo!(),
        }
    }

    fn fmt_stmt(&mut self, stmt: &ast::RawStmt) {
        match stmt.kind {
            RawStmtKind::Loop(_) => todo!(),
            RawStmtKind::Branch(_) => todo!(),
            RawStmtKind::Exec(expr) => self.fmt_expr(&self.arena[expr]),
            RawStmtKind::Init {
                kind,
                varname,
                initial_value,
            } => {
                match kind {
                    ast::InitKind::Let => self.write("let "),
                    ast::InitKind::Var => self.write("var "),
                    ast::InitKind::Functional => (),
                }
                self.fmt_ident(varname.into());
                self.write(" = ");
                self.fmt_expr(&self.arena[initial_value]);
            }
            RawStmtKind::Return(expr) => {
                self.write("return ");
                self.fmt_expr(&self.arena[expr]);
            }
            RawStmtKind::Assert(expr) => {
                self.write("assert ");
                self.fmt_expr(&self.arena[expr]);
            }
        }
    }

    fn fmt_expr(&mut self, expr: &RawExpr) {
        match expr.kind {
            RawExprKind::Variable(ident) => self.write(&ident),
            RawExprKind::Literal(literal) => match literal {
                PrimitiveValue::I32(i) => self.write(&i.to_string()),
                PrimitiveValue::F32(f) => self.write(&f.to_string()),
                PrimitiveValue::Void => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::Bool(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
            },
            RawExprKind::Bracketed(expr_idx) => {
                self.write("(");
                self.fmt_expr(&self.arena[expr_idx]);
                self.write(")");
            }
            RawExprKind::Opn { opr: opn, ref opds } => match opn {
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
            RawExprKind::Scope { .. } => todo!(),
            RawExprKind::Lambda(ref inputs, expr) => {
                self.write("|");
                self.join(
                    inputs,
                    |this, (ident, ty)| {
                        this.fmt_ident((*ident).into());
                        if let Some(ty) = ty {
                            this.write(": ");
                            this.fmt_type(*ty)
                        }
                    },
                    ", ",
                );
                self.write("| ");
                self.fmt_expr(&self.arena[expr])
            }
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
