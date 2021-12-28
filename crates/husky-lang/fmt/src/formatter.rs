use std::ops::AddAssign;

use common::*;

use ast::{AstGenResult, Expr, ExprError, ExprKind};
use atom::{PrefixOpr, StmtAttr};
use folded::FoldedIdx;
use hir::*;
use scope::ScopeId;
use word::{BuiltinIdentifier, WordInterner};

use crate::*;

pub struct Formatter<'a> {
    word_interner: &'a WordInterner,
    arena: &'a ast::ExprArena,
    formatted_text: FormattedText,
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(word_interner: &'a WordInterner, arena: &'a ast::ExprArena) -> Self {
        Self {
            word_interner,
            arena,
            formatted_text: FormattedText::new(),
        }
    }

    pub(crate) fn finish(self) -> FormattedText {
        self.formatted_text
    }
}

pub struct Writer {
    indent: folded::Indent,
    result: String,
}

impl Writer {
    fn new(indent: folded::Indent) -> Self {
        let mut writer = Writer {
            indent,
            result: String::new(),
        };
        writer.newline();
        writer
    }

    fn newline(&mut self) {
        self.result
            .reserve(self.result.len() + self.indent as usize);
        for _ in 0..self.indent {
            self.result.push(' ');
        }
    }

    fn write(&mut self, s: &str) {
        self.result += s;
    }

    fn finish(self) -> String {
        self.result
    }
}

impl<'a> folded::Transformer<AstGenResult, folded::FoldedList<AstGenResult>, ast::AstResult<String>>
    for Formatter<'a>
{
    fn enter(&mut self) {}

    fn exit(&mut self) {}

    fn post_exit(&mut self, _: FoldedIdx<ast::AstResult<String>>) {}

    fn transform(
        &mut self,
        indent: folded::Indent,
        input: &AstGenResult,
    ) -> ast::AstResult<String> {
        Ok(self.fmt(input.as_ref().map_err(|e| e.clone())?, Writer::new(indent)))
    }

    fn folded_outputs_mut(&mut self) -> &mut FormattedText {
        &mut self.formatted_text
    }
}

impl<'a> Formatter<'a> {
    fn fmt(&self, ast: &ast::Ast, mut writer: Writer) -> String {
        match ast {
            ast::Ast::TypeDef {
                ident,
                kind,
                placeholders: args,
            } => {
                epin!();
                match kind {
                    TypeKind::Enum(_) => todo!(),
                    TypeKind::Struct => writer.write("struct "),
                }
                self.fmt_ident(&mut writer, ident.into());
                if args.len() > 0 {
                    todo!()
                }
            }
            ast::Ast::MainDef => writer.write("main:"),
            ast::Ast::FuncDef { kind, decl } => {
                writer.write(match kind {
                    FuncKind::Test => "test ",
                    FuncKind::Proc => todo!(),
                    FuncKind::PureFunc => "func ",
                    FuncKind::Def => todo!(),
                });
                self.word_interner
                    .apply(decl.funcname.into(), |s| writer.write(s));
                writer.write("(");
                for i in 0..decl.inputs.len() {
                    if i > 0 {
                        writer.write(", ");
                    }
                    let (ident, ty) = &decl.inputs[i];
                    self.fmt_ident(&mut writer, ident.into());
                    writer.write(": ");
                    self.fmt_func_input_contracted_type(&mut writer, ty);
                }
                writer.write(")");
                if decl.output != ScopeId::Builtin(BuiltinIdentifier::Void) {
                    writer.write(" -> ");
                    self.fmt_type(&mut writer, decl.output);
                }
                writer.write(":");
            }
            ast::Ast::PatternDef => todo!(),
            ast::Ast::Use { ident, scope } => todo!(),
            ast::Ast::MembDef { ident, kind } => match kind {
                MembKind::MembVar { ty } => {
                    self.fmt_ident(&mut writer, ident.into());
                    writer.write(": ");
                    self.fmt_member_variable_contracted_type(&mut writer, ty);
                }
                MembKind::MembFunc {
                    this,
                    inputs,
                    output,
                    args,
                } => todo!(),
            },
            ast::Ast::Stmt(stmt) => self.fmt_stmt(&mut writer, stmt),
        }
        writer.finish()
    }

    fn fmt_ident(&self, writer: &mut Writer, ident: word::Identifier) {
        self.word_interner
            .apply(word::Word::Identifier(ident), |s: &str| writer.write(s))
    }

    fn fmt_member_variable_contracted_type(&self, writer: &mut Writer, ty: &InputType) {
        match ty.contract {
            InputContract::Intact => todo!(),
            InputContract::Share => todo!(),
            InputContract::Own => (),
        }
        self.fmt_type(writer, ty.ty);
    }

    fn fmt_func_input_contracted_type(&self, writer: &mut Writer, ty: &InputType) {
        match ty.contract {
            InputContract::Intact => (),
            InputContract::Share => writer.write("&"),
            InputContract::Own => writer.write("!"),
        }
        self.fmt_type(writer, ty.ty);
    }

    fn fmt_type(&self, writer: &mut Writer, ty: ScopeId) {
        match ty {
            ScopeId::Builtin(ident) => writer.write(ident.code()),
            ScopeId::Custom(_) => todo!(),
        }
    }

    fn fmt_stmt(&self, writer: &mut Writer, stmt: &ast::Stmt) {
        match stmt {
            ast::Stmt::Loop(_) => todo!(),
            ast::Stmt::Branch(_) => todo!(),
            ast::Stmt::Exec { expr } => todo!(),
            ast::Stmt::Init {
                kind,
                varname,
                initial_value,
            } => {
                match kind {
                    ast::InitKind::Let => writer.write("let "),
                    ast::InitKind::Var => writer.write("var "),
                    ast::InitKind::Functional => (),
                }
                self.fmt_ident(writer, varname.into());
                writer.write(" = ");
                self.fmt_expr(writer, &self.arena[initial_value]);
            }
            ast::Stmt::Return(expr) => {
                writer.write("return ");
                self.fmt_expr(writer, &self.arena[expr]);
            }
            ast::Stmt::Assert(expr) => {
                writer.write("assert ");
                self.fmt_expr(writer, &self.arena[expr]);
            }
        }
    }

    fn fmt_expr(&self, writer: &mut Writer, expr: &Expr) {
        match &expr.kind {
            ExprKind::Variable(ident) => self
                .word_interner
                .apply(word::Word::Identifier(*ident), |s| writer.write(s)),
            ExprKind::Literal(literal) => match literal {
                atom::Literal::I32Literal(i) => writer.write(&i.to_string()),
                atom::Literal::F32Literal(f) => writer.write(&f.to_string()),
            },
            ExprKind::Bracketed(expr_idx) => {
                writer.write("(");
                self.fmt_expr(writer, &self.arena[expr_idx]);
                writer.write(")");
            }
            ExprKind::Opn { opr: opn, opds } => match opn {
                ast::Opr::Binary(opr) => {
                    let opds = &self.arena[opds];
                    self.fmt_expr(writer, &opds[0]);
                    writer.write(opr.spaced_code());
                    self.fmt_expr(writer, &opds[1]);
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
                //     writer.write("(");
                //     for i in 1..opds.len() {
                //         if i >= 2 {
                //             writer.write(", ")
                //         }
                //         self.fmt_expr(result, &opds[i]);
                //     }
                //     writer.write(")");
                // }
                // ast::Opr::MemberCall(_) => todo!(),
                // ast::Opr::Member(_) => todo!(),
                // ast::Opr::Index => todo!(),
                // ast::Opr::Opr(opr) => match opr {},
            },
            ExprKind::Void => writer.write("()"),
            ExprKind::Scope(_) => todo!(),
        }
    }
}
