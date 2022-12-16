use std::ops::AddAssign;

use husky_ast::*;
use husky_defn_head::Parameter;
use husky_entity_kind::TyKind;
use husky_entity_tree::EntityTreeDb;
use husky_expr::*;
use husky_init_syntax::InitKind;
use husky_print_utils::msg_once;
use husky_term::Term;
use husky_token::ParadigmKeyword;

pub struct Formatter<'a> {
    db: &'a dyn EntityTreeDb,
    arena: &'a ExprArena,
    // indent: fold::Indent,
    result: String,
    // context: LocalValue<AstContext>,
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(db: &'a dyn EntityTreeDb, arena: &'a ExprArena, context: AstContext) -> Self {
        Self {
            db,
            arena,
            // indent: 0,
            result: String::new(),
            // context: LocalValue::new(context),
        }
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}

impl<'a> Formatter<'a> {
    fn newline(&mut self) {
        todo!()
        // self.result
        //     .reserve_exact(self.result.len() + self.indent as usize + 1);
        // self.result.push('\n');
        // for _ in 0..self.indent {
        //     self.result.push(' ');
        // }
    }

    fn write(&mut self, s: &str) {
        self.result += s;
    }
}

impl<'a> Formatter<'a> {
    fn fmt(&mut self, ast: &husky_ast::Ast, enter_block: impl FnOnce(&mut Self)) {
        todo!()
        // match ast.variant {
        //     DeprecatedAstVariant::TypeDefnHead {
        //         ident,
        //         ref kind,
        //         ref spatial_parameters,
        //     } => {
        //         enter_block(self);
        //         match kind {
        //             TyKind::Enum => todo!(),
        //             TyKind::Struct => {
        //                 self.context.set(AstContext::Struct {
        //                     item_context: StructItemContext::OriginalField,
        //                     opt_base_ty: None,
        //                 });
        //                 self.write("struct ")
        //             }
        //             TyKind::Record => todo!(),
        //             TyKind::Primitive => todo!(),
        //             TyKind::Vec => todo!(),
        //             TyKind::Array => todo!(),
        //             TyKind::Slice => todo!(),
        //             TyKind::CyclicSlice => todo!(),
        //             TyKind::Tuple => todo!(),
        //             TyKind::Mor => todo!(),
        //             TyKind::ThickFp => todo!(),
        //             TyKind::AssociatedAny => todo!(),
        //             TyKind::TargetOutputAny => todo!(),
        //             TyKind::ThisAny => todo!(),
        //             TyKind::SpatialPlaceholderAny => todo!(),
        //             TyKind::BoxAny => todo!(),
        //             TyKind::HigherKind => todo!(),
        //             TyKind::Ref => todo!(),
        //             TyKind::Option => todo!(),
        //         }
        //         self.fmt_ident(ident.ident.into());
        //         if spatial_parameters.len() > 0 {
        //             todo!()
        //         }
        //     }
        //     DeprecatedAstVariant::MainDefnHead => {
        //         enter_block(self);
        //         self.context.set(AstContext::Stmt {
        //             paradigm: Paradigm::LazyFunctional,
        //             return_context: Some(RawReturnContext {
        //                 opt_return_ty: todo!(),
        //                 //  Some(Term {
        //                 //     route: self.db.intern_entity_route(EntityRoute {
        //                 //         variant: EntityRouteVariant::TargetOutputType,
        //                 //         temporal_arguments: Default::default(),
        //                 //         spatial_arguments: Default::default(),
        //                 //     }),
        //                 //     range: Default::default(),
        //                 // }),
        //                 kind: RawReturnContextKind::Feature,
        //             }),
        //         });
        //         self.write("main:")
        //     }
        //     DeprecatedAstVariant::CallFormDefnHead {
        //         paradigm,
        //         ident,
        //         ref parameters,
        //         return_ty,
        //         ..
        //     } => {
        //         enter_block(self);
        //         self.context.set(AstContext::Stmt {
        //             paradigm,
        //             return_context: Some(RawReturnContext {
        //                 opt_return_ty: Some(return_ty),
        //                 kind: RawReturnContextKind::Normal,
        //             }),
        //         });
        //         self.write(match paradigm {
        //             Paradigm::EagerProcedural => "proc ",
        //             Paradigm::EagerFunctional => "func ",
        //             Paradigm::LazyFunctional => todo!(),
        //         });
        //         todo!()
        //         // msg_once!("generic parameters");
        //         // self.write(&ident.ident);
        //         // self.write("(");
        //         // for i in 0..parameters.len() {
        //         //     if i > 0 {
        //         //         self.write(", ");
        //         //     }
        //         //     let parameter = &parameters[i];
        //         //     self.fmt_parameter(parameter);
        //         // }
        //         // self.write(")");
        //         // todo!();
        //         // // if return_ty.route != Term::Root(RootBuiltinIdentifier::Void) {
        //         // //     self.write(" -> ");
        //         // //     self.fmt_ty(return_ty.route);
        //         // // }
        //         // self.write(":");
        //     }
        //     DeprecatedAstVariant::FieldDefnHead {
        //         ranged_ident,
        //         field_ty,
        //         ..
        //     } => {
        //         todo!()
        //         // match liason {
        //         //     MemberModifier::Immutable => (),
        //         //     MemberModifier::Mutable => todo!(),
        //         //     MemberModifier::Property => todo!(),
        //         // }
        //         // self.fmt_ident(ranged_ident.ident.into());
        //         // self.write(": ");
        //         // self.fmt_ty(field_ty.route)
        //     }
        //     DeprecatedAstVariant::Stmt(ref stmt) => self.fmt_stmt(stmt),
        //     DeprecatedAstVariant::DatasetConfigDefnHead => todo!(),
        //     DeprecatedAstVariant::EnumVariantDefnHead { .. } => todo!(),
        //     DeprecatedAstVariant::FeatureDefnHead { .. } => todo!(),
        //     DeprecatedAstVariant::Use { .. } => todo!(),
        //     DeprecatedAstVariant::Submodule { .. } => todo!(),
        //     DeprecatedAstVariant::Visual => todo!(),
        // }
    }

    fn fmt_ident(&mut self, ident: husky_word::Identifier) {
        todo!()
        // self.result.add_assign(&ident)
    }

    fn fmt_parameter(&mut self, parameter: &Parameter) {
        todo!()
        // match parameter.liason() {
        //     ParameterModifier::None => (),
        //     ParameterModifier::EvalRef => self.write("&"),
        //     ParameterModifier::Owned => self.write("!!"),
        //     ParameterModifier::TempRefMut => self.write("mut"),
        //     ParameterModifier::OwnedMut => self.write("mut !!"),
        //     ParameterModifier::MemberAccess => todo!(),
        //     ParameterModifier::TempRef => todo!(),
        // }
        // self.fmt_ident(parameter.ident().into());
        // self.write(": ");
        // self.fmt_ty(parameter.raw_ty());
    }

    fn fmt_ty(&mut self, ty: Term) {
        todo!()
        // match ty {
        //     Term::Root(ident) => self.write(&ident),
        //     Term::Custom(_) => todo!(),
        // }
    }

    fn fmt_stmt(&mut self, stmt: &husky_ast::RawStmt) {
        match stmt.variant {
            RawStmtVariant::Loop(_) => todo!(),
            RawStmtVariant::IfElseBranch { .. } => todo!(),
            RawStmtVariant::MatchBranch { .. } => todo!(),
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
                todo!()
                // match self.context.value() {
                //     AstContext::Stmt {
                //         paradigm: Paradigm::EagerFunctional,
                //         ..
                //     }
                //     | AstContext::Stmt {
                //         paradigm: Paradigm::LazyFunctional,
                //         ..
                //     } => (),
                //     AstContext::Stmt {
                //         paradigm: Paradigm::EagerProcedural,
                //         ..
                //     } => self.write("return "),
                //     _ => panic!(),
                // }
                // self.fmt_expr(&self.arena[result]);
            }
            RawStmtVariant::Assert(expr) => {
                self.write("assert ");
                self.fmt_expr(&self.arena[expr]);
            }
            RawStmtVariant::Break => todo!(),
            RawStmtVariant::Match { .. } => todo!(),
            RawStmtVariant::ReturnXml(_) => todo!(),
            RawStmtVariant::Require { .. } => todo!(),
        }
    }

    fn fmt_expr(&mut self, _expr: &Expr) {
        // match expr.variant {
        //     RawExprVariant::Variable { varname, .. } => self.write(&varname),
        //     RawExprVariant::Unrecognized(varname) => self.write(&varname),
        //     RawExprVariant::PrimitiveLiteral(literal) => match literal {
        //         RawLiteralData::Integer(i) => self.write(&i.to_string()),
        //         RawLiteralData::I32(i) => self.write(&i.to_string()),
        //         RawLiteralData::Float(f) => self.write(&f.to_string()),
        //         RawLiteralData::F32(f) => self.write(&f.to_string()),
        //         RawLiteralData::Void => todo!(),
        //         RawLiteralData::I64(_) => todo!(),
        //         RawLiteralData::F64(_) => todo!(),
        //         RawLiteralData::Bits(_) => todo!(),
        //         RawLiteralData::B32(_) => todo!(),
        //         RawLiteralData::B64(_) => todo!(),
        //         RawLiteralData::Bool(_) => todo!(),
        //     },
        //     RawExprVariant::Bracketed(raw_expr_idx) => {
        //         self.write("(");
        //         self.fmt_expr(&self.arena[raw_expr_idx]);
        //         self.write(")");
        //     }
        //     RawExprVariant::Opn {
        //         opn_variant: ref opr,
        //         ref opds,
        //     } => match opr {
        //         RawOpnVariant::Binary(opr) => {
        //             let opds = &self.arena[opds];
        //             self.fmt_expr(&opds[0]);
        //             self.write(opr.spaced_code());
        //             self.fmt_expr(&opds[1]);
        //         }
        //         RawOpnVariant::Prefix(_) => todo!(),
        //         RawOpnVariant::Suffix(_) => todo!(),
        //         RawOpnVariant::List(opr) => match opr {
        //             ListOpr::NewTuple => todo!(),
        //             ListOpr::NewVec => todo!(),
        //             ListOpr::NewDict => todo!(),
        //             ListOpr::FunctionCall => todo!(),
        //             ListOpr::Index => todo!(),
        //             ListOpr::ModuloIndex => todo!(),
        //             ListOpr::StructInit => todo!(),
        //             ListOpr::MethodCall { .. } => todo!(),
        //         },
        //         RawOpnVariant::Field(_) => todo!(),
        //     },
        //     RawExprVariant::Entity { .. } => todo!(),
        //     RawExprVariant::Lambda(ref inputs, expr) => {
        //         self.write("|");
        //         self.join(
        //             inputs,
        //             |this, (ident, ty)| {
        //                 this.fmt_ident((*ident).ident.into());
        //                 if let Some(ty) = ty {
        //                     this.write(": ");
        //                     this.fmt_ty(ty.route)
        //                 }
        //             },
        //             ", ",
        //         );
        //         self.write("| ");
        //         self.fmt_expr(&self.arena[expr])
        //     }
        //     RawExprVariant::ThisValue { .. } => todo!(),
        //     RawExprVariant::FrameVariable { .. } => todo!(),
        //     RawExprVariant::ThisField { .. } => todo!(),
        // }
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
