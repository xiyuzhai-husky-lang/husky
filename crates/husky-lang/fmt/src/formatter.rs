use std::ops::AddAssign;

use common::*;

use ast::{Ast, AstResult, Expr, ExprKind};
use fold::FoldIdx;
use hir::*;
use scope::ScopeId;
use word::{BuiltinIdentifier, WordInterner};

use crate::*;

pub struct Formatter<'a> {
    word_interner: &'a WordInterner,
    arena: &'a ast::ExprArena,
    indent: fold::Indent,
    result: String,
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(word_interner: &'a WordInterner, arena: &'a ast::ExprArena) -> Self {
        Self {
            word_interner,
            arena,
            indent: 0,
            result: String::new(),
        }
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}

impl<'a> fold::Transcriber<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>> for Formatter<'a> {
    fn enter_fold(&mut self) {}

    fn enter_block(&mut self) {}

    fn exit(&mut self) {}

    fn transcribe(&mut self, indent: fold::Indent, input: &AstResult<Ast>) {
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
                    TypeKind::Enum(_) => todo!(),
                    TypeKind::Struct => self.write("struct "),
                }
                self.fmt_ident(ident.into());
                if generics.len() > 0 {
                    todo!()
                }
            }
            ast::Ast::MainDef => self.write("main:"),
            ast::Ast::FuncDef { kind, decl } => {
                self.write(match kind {
                    FuncKind::Test => "test ",
                    FuncKind::Proc => todo!(),
                    FuncKind::PureFunc => "func ",
                    FuncKind::Def => todo!(),
                });
                self.word_interner
                    .apply(decl.funcname.into(), |s| self.write(s));
                self.write("(");
                for i in 0..decl.inputs.len() {
                    if i > 0 {
                        self.write(", ");
                    }
                    let (ident, ty) = &decl.inputs[i];
                    self.fmt_ident(ident.into());
                    self.write(": ");
                    self.fmt_func_input_contracted_type(ty);
                }
                self.write(")");
                if decl.output != ScopeId::Builtin(BuiltinIdentifier::Void) {
                    self.write(" -> ");
                    self.fmt_type(decl.output);
                }
                self.write(":");
            }
            ast::Ast::PatternDef => todo!(),
            ast::Ast::Use { ident, scope } => todo!(),
            ast::Ast::MembDef { ident, kind } => match kind {
                MembKind::MembVar { ty } => {
                    self.fmt_ident(ident.into());
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
        self.word_interner
            .apply(word::Word::Identifier(ident), |s: &str| {
                self.result.add_assign(s)
            })
    }

    fn fmt_member_variable_contracted_type(&mut self, ty: &InputType) {
        match ty.contract {
            InputContract::Intact => todo!(),
            InputContract::Share => todo!(),
            InputContract::Own => (),
        }
        self.fmt_type(ty.ty);
    }

    fn fmt_func_input_contracted_type(&mut self, ty: &InputType) {
        match ty.contract {
            InputContract::Intact => (),
            InputContract::Share => self.write("&"),
            InputContract::Own => self.write("!"),
        }
        self.fmt_type(ty.ty);
    }

    fn fmt_type(&mut self, ty: ScopeId) {
        match ty {
            ScopeId::Builtin(ident) => self.write(ident.code()),
            ScopeId::Custom(_) => todo!(),
        }
    }

    fn fmt_stmt(&mut self, stmt: &ast::Stmt) {
        match stmt {
            ast::Stmt::Loop(_) => todo!(),
            ast::Stmt::Branch(_) => todo!(),
            ast::Stmt::Exec(expr) => self.fmt_expr(&self.arena[expr]),
            ast::Stmt::Init {
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
            ast::Stmt::Return(expr) => {
                self.write("return ");
                self.fmt_expr(&self.arena[expr]);
            }
            ast::Stmt::Assert(expr) => {
                self.write("assert ");
                self.fmt_expr(&self.arena[expr]);
            }
        }
    }

    fn fmt_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Variable(ident) => self
                .word_interner
                .apply(word::Word::Identifier(*ident), |s| self.write(s)),
            ExprKind::Literal(literal) => match literal {
                hir::Literal::I32Literal(i) => self.write(&i.to_string()),
                hir::Literal::F32Literal(f) => self.write(&f.to_string()),
            },
            ExprKind::Bracketed(expr_idx) => {
                self.write("(");
                self.fmt_expr(&self.arena[expr_idx]);
                self.write(")");
            }
            ExprKind::Opn { opr: opn, opds } => match opn {
                ast::Opr::Binary(opr) => {
                    let opds = &self.arena[opds];
                    self.fmt_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.fmt_expr(&opds[1]);
                }
                ast::Opr::Prefix(opr) => todo!(),
                ast::Opr::Suffix(_) => todo!(),
                ast::Opr::List(opr) => match opr {
                    ast::ListOpr::TupleInit => todo!(),
                    ast::ListOpr::NewVec => todo!(),
                    ast::ListOpr::NewDict => todo!(),
                    ast::ListOpr::Call => todo!(),
                    ast::ListOpr::Index => todo!(),
                    ast::ListOpr::ModuloIndex => todo!(),
                    ast::ListOpr::StructInit => todo!(),
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
            ExprKind::Void => self.write("()"),
            ExprKind::Scope(_) => todo!(),
            ExprKind::Lambda(inputs, expr) => {
                self.write("|");
                self.join(
                    inputs,
                    |this, (ident, ty)| {
                        this.fmt_ident(ident.into());
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
