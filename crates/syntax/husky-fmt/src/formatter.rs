use std::ops::AddAssign;

use fold::LocalValue;
use husky_ast::{
    Ast, AstContext, AstQueryGroup, AstResult, AstVariant, RawExpr, RawExprVariant,
    RawReturnContext, RawReturnContextKind, RawStmtVariant, StructItemContext,
};
use husky_defn_head::Parameter;
use husky_entity_kind::TyKind;
use husky_entity_route::{EntityRoute, EntityRoutePtr, EntityRouteVariant, RangedEntityRoute};
use husky_entity_syntax::EntitySyntaxQueryGroup;
use husky_init_syntax::InitKind;
use husky_liason_semantics::{MemberLiason, ParameterLiason};
use husky_opn_syntax::{ListOpr, RawOpnVariant};
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_print_utils::msg_once;
use husky_vm::*;
use husky_word::{Paradigm, RootIdentifier, WordInterner};

pub struct Formatter<'a> {
    db: &'a dyn EntitySyntaxQueryGroup,
    arena: &'a husky_ast::RawExprArena,
    indent: fold::Indent,
    result: String,
    context: LocalValue<AstContext>,
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(
        db: &'a dyn EntitySyntaxQueryGroup,
        arena: &'a husky_ast::RawExprArena,
        context: AstContext,
    ) -> Self {
        Self {
            db,
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

impl<'a> fold::Executor<AstResult<Ast>, fold::FoldableList<AstResult<Ast>>> for Formatter<'a> {
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
    fn fmt(&mut self, ast: &husky_ast::Ast, enter_block: impl FnOnce(&mut Self)) {
        match ast.variant {
            AstVariant::TypeDefnHead {
                ident,
                ref kind,
                ref spatial_parameters,
            } => {
                enter_block(self);
                match kind {
                    TyKind::Enum => todo!(),
                    TyKind::Struct => {
                        let opt_base_ty = self.context.value().opt_subroute(self.db, ident.ident);
                        self.context.set(AstContext::Struct {
                            item_context: StructItemContext::OriginalField,
                            opt_base_ty: None,
                        });
                        self.write("struct ")
                    }
                    TyKind::Record => todo!(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Slice => todo!(),
                    TyKind::CyclicSlice => todo!(),
                    TyKind::Tuple => todo!(),
                    TyKind::Mor => todo!(),
                    TyKind::ThickFp => todo!(),
                    TyKind::AssociatedAny => todo!(),
                    TyKind::TargetOutputAny => todo!(),
                    TyKind::ThisAny => todo!(),
                    TyKind::SpatialPlaceholderAny => todo!(),
                    TyKind::BoxAny => todo!(),
                    TyKind::HigherKind => todo!(),
                    TyKind::Ref => todo!(),
                    TyKind::Option => todo!(),
                }
                self.fmt_ident(ident.ident.into());
                if spatial_parameters.len() > 0 {
                    todo!()
                }
            }
            AstVariant::MainDefnHead => {
                enter_block(self);
                self.context.set(AstContext::Stmt {
                    paradigm: Paradigm::LazyFunctional,
                    return_context: Some(RawReturnContext {
                        return_ty: RangedEntityRoute {
                            route: self.db.intern_entity_route(EntityRoute {
                                variant: EntityRouteVariant::TargetOutputType,
                                temporal_arguments: Default::default(),
                                spatial_arguments: Default::default(),
                            }),
                            range: Default::default(),
                        },
                        kind: RawReturnContextKind::Feature,
                    }),
                });
                self.write("main:")
            }
            AstVariant::CallFormDefnHead {
                paradigm,
                ident,
                ref parameters,
                output_ty,
                ..
            } => {
                enter_block(self);
                self.context.set(AstContext::Stmt {
                    paradigm,
                    return_context: Some(RawReturnContext {
                        return_ty: output_ty,
                        kind: RawReturnContextKind::Normal,
                    }),
                });
                self.write(match paradigm {
                    Paradigm::EagerProcedural => "proc ",
                    Paradigm::EagerFunctional => "func ",
                    Paradigm::LazyFunctional => todo!(),
                });
                msg_once!("generic parameters");
                self.write(&ident.ident);
                self.write("(");
                for i in 0..parameters.len() {
                    if i > 0 {
                        self.write(", ");
                    }
                    let input_placeholder = &parameters[i];
                    self.fmt_ident(input_placeholder.ranged_ident.ident.into());
                    self.write(": ");
                    self.fmt_func_input_liasoned_type(input_placeholder);
                }
                self.write(")");
                if output_ty.route != EntityRoutePtr::Root(RootIdentifier::Void) {
                    self.write(" -> ");
                    self.fmt_ty(output_ty.route);
                }
                self.write(":");
            }
            AstVariant::FieldDefnHead {
                liason,
                ranged_ident,
                field_ty: ty,
                field_ast_kind: field_kind,
            } => {
                match liason {
                    MemberLiason::Immutable => (),
                    MemberLiason::Mutable => todo!(),
                    MemberLiason::DerivedLazy => todo!(),
                }
                self.fmt_ident(ranged_ident.ident.into());
                self.write(": ");
                self.fmt_ty(ty.route)
            }
            AstVariant::Stmt(ref stmt) => self.fmt_stmt(stmt),
            AstVariant::DatasetConfigDefnHead => todo!(),
            AstVariant::EnumVariantDefnHead {
                ident,
                variant_class: ref variant_kind,
            } => todo!(),
            AstVariant::FeatureDefnHead { .. } => todo!(),
            AstVariant::Use { ref use_variant } => todo!(),
            AstVariant::Submodule { ident, source_file } => todo!(),
            AstVariant::CallFormDefnHead { .. } => todo!(),
            AstVariant::Visual => todo!(),
        }
    }

    fn fmt_ident(&mut self, ident: husky_word::Identifier) {
        self.result.add_assign(&ident)
    }

    fn fmt_func_input_liasoned_type(&mut self, ty: &Parameter) {
        match ty.ranged_liason.liason {
            ParameterLiason::Pure => (),
            ParameterLiason::EvalRef => self.write("&"),
            ParameterLiason::Move => self.write("!!"),
            ParameterLiason::TempRefMut => self.write("mut"),
            ParameterLiason::MoveMut => self.write("mut !!"),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::TempRef => todo!(),
        }
        self.fmt_ty(ty.ranged_ty.route);
    }

    fn fmt_ty(&mut self, ty: EntityRoutePtr) {
        match ty {
            EntityRoutePtr::Root(ident) => self.write(&ident),
            EntityRoutePtr::Custom(_) => todo!(),
        }
    }

    fn fmt_stmt(&mut self, stmt: &husky_ast::RawStmt) {
        match stmt.variant {
            RawStmtVariant::Loop(_) => todo!(),
            RawStmtVariant::ConditionBranch { .. } => todo!(),
            RawStmtVariant::PatternBranch { .. } => todo!(),
            RawStmtVariant::Exec {
                expr,
                discard: silent,
            } => {
                self.fmt_expr(&self.arena[expr]);
                if silent {
                    self.write(";")
                }
            }
            RawStmtVariant::Init {
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
            RawStmtVariant::Return { result, .. } => {
                match self.context.value() {
                    AstContext::Stmt {
                        paradigm: Paradigm::EagerFunctional,
                        return_context,
                    }
                    | AstContext::Stmt {
                        paradigm: Paradigm::LazyFunctional,
                        return_context,
                    }
                    | AstContext::Stmt {
                        paradigm: Paradigm::LazyFunctional,
                        return_context,
                    } => (),
                    AstContext::Stmt {
                        paradigm: Paradigm::EagerProcedural,
                        ..
                    } => self.write("return "),
                    _ => panic!(),
                }
                self.fmt_expr(&self.arena[result]);
            }
            RawStmtVariant::Assert(expr) => {
                self.write("assert ");
                self.fmt_expr(&self.arena[expr]);
            }
            RawStmtVariant::Break => todo!(),
            RawStmtVariant::Match { .. } => todo!(),
            RawStmtVariant::ReturnXml(_) => todo!(),
            RawStmtVariant::Require { condition, .. } => todo!(),
        }
    }

    fn fmt_expr(&mut self, expr: &RawExpr) {
        match expr.variant {
            RawExprVariant::Variable { varname, .. } => self.write(&varname),
            RawExprVariant::Unrecognized(varname) => self.write(&varname),
            RawExprVariant::PrimitiveLiteral(literal) => match literal {
                PrimitiveLiteralData::Integer(i) => self.write(&i.to_string()),
                PrimitiveLiteralData::I32(i) => self.write(&i.to_string()),
                PrimitiveLiteralData::Float(f) => self.write(&f.to_string()),
                PrimitiveLiteralData::F32(f) => self.write(&f.to_string()),
                PrimitiveLiteralData::Void => todo!(),
                PrimitiveLiteralData::I64(_) => todo!(),
                PrimitiveLiteralData::F64(_) => todo!(),
                PrimitiveLiteralData::Bits(_) => todo!(),
                PrimitiveLiteralData::B32(_) => todo!(),
                PrimitiveLiteralData::B64(_) => todo!(),
                PrimitiveLiteralData::Bool(_) => todo!(),
            },
            RawExprVariant::Bracketed(raw_expr_idx) => {
                self.write("(");
                self.fmt_expr(&self.arena[raw_expr_idx]);
                self.write(")");
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => match opr {
                RawOpnVariant::Binary(opr) => {
                    let opds = &self.arena[opds];
                    self.fmt_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.fmt_expr(&opds[1]);
                }
                RawOpnVariant::Prefix(opr) => todo!(),
                RawOpnVariant::Suffix(_) => todo!(),
                RawOpnVariant::List(opr) => match opr {
                    ListOpr::TupleInit => todo!(),
                    ListOpr::NewVec => todo!(),
                    ListOpr::NewDict => todo!(),
                    ListOpr::FunctionCall => todo!(),
                    ListOpr::Index => todo!(),
                    ListOpr::ModuloIndex => todo!(),
                    ListOpr::StructInit => todo!(),
                    ListOpr::MethodCall { .. } => todo!(),
                },
                RawOpnVariant::Field(_) => todo!(),
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
            RawExprVariant::ThisValue { .. } => todo!(),
            RawExprVariant::FrameVariable {
                varname,
                init_range: init_row,
            } => todo!(),
            RawExprVariant::ThisField { .. } => todo!(),
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
