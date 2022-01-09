use fold::{FoldedList, LocalStack, LocalValue};
use hir::*;
use text::TextRanged;
use token::*;
use word::*;

use crate::{
    atom::parser::ScopeLRParser,
    atom::symbol_proxy::{Symbol, SymbolProxy},
    query::{AstQuery, AstText},
    *,
};

pub struct AstTransformer<'a> {
    db: &'a dyn AstQuery,
    arena: ExprArena,
    folded_results: FoldedList<AstResult<Ast>>,
    symbols: LocalStack<Symbol>,
    env: LocalValue<hir::Env>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(db: &'a dyn AstQuery, module: scope::PackageOrModule) -> Self {
        Self {
            db,
            arena: ExprArena::new(),
            folded_results: FoldedList::new(),
            symbols: LocalStack::new(),
            env: LocalValue::new(match db.id_to_scope(module.scope_id()).route {
                scope::ScopeRoute::Builtin(_) => todo!(),
                scope::ScopeRoute::Package(_, _) => Env::Package,
                scope::ScopeRoute::ChildScope(_, _) => Env::Module,
            }),
        }
    }

    pub(crate) fn finish(self) -> AstText {
        AstText {
            arena: self.arena,
            folded_results: self.folded_results,
        }
    }

    fn scope_proxy(&mut self) -> SymbolProxy {
        SymbolProxy {
            db: self.db,
            symbols: &self.symbols,
            line: 0,
        }
    }
}

pub struct AtomTask {}

impl<'a> fold::Transformer<[Token], TokenizedText, AstResult<Ast>, AtomTask>
    for AstTransformer<'a>
{
    fn _enter_block(&mut self) {
        self.env.enter();
        self.symbols.enter();
    }

    fn _exit(&mut self) {
        self.env.exit();
        self.symbols.exit();
    }

    fn transform(
        &mut self,
        indent: fold::Indent,
        tokens: &[Token],
        enter_block: &mut impl FnOnce(&mut Self),
    ) -> AstResult<Ast> {
        if let TokenKind::Keyword(keyword) = tokens[0].kind {
            match keyword {
                Keyword::Func(func_kw) => match func_kw {
                    word::FuncKeyword::Main => Ok(Ast::MainDef),
                    word::FuncKeyword::Test => todo!(),
                    word::FuncKeyword::Proc => todo!(),
                    word::FuncKeyword::Func => {
                        let mut parser =
                            ScopeLRParser::new(self.scope_proxy(), &tokens[1..(tokens.len() - 1)]);
                        let decl = parser.func_decl()?;
                        Ok(Ast::FuncDef {
                            kind: FuncKind::PureFunc,
                            decl,
                        })
                    }
                    word::FuncKeyword::Def => todo!(),
                    word::FuncKeyword::Pattern => todo!(),
                },
                Keyword::Type(ty_kw) => match ty_kw {
                    word::TypeKeyword::Struct => {
                        if tokens.len() == 1 {
                            // need to know where the keyword is
                            todo!()
                        }

                        let ident = match tokens[1].kind {
                            TokenKind::Keyword(_) => todo!(),
                            TokenKind::Identifier(ident) => match ident {
                                Identifier::Builtin(_) => ast_err!(
                                    text::group_text_range(&tokens[1..]),
                                    "expect custom identifier, but got builtin instead"
                                )?,
                                Identifier::Custom(custom_ident) => custom_ident,
                            },
                            TokenKind::Special(_) => todo!(),
                            TokenKind::I32Literal(_) => todo!(),
                            TokenKind::F32Literal(_) => todo!(),
                        };
                        if tokens.len() > 3 {
                            todo!()
                        }
                        if tokens.last().unwrap().kind != TokenKind::Special(Special::Colon) {
                            todo!()
                        }
                        Ok(Ast::TypeDef {
                            ident,
                            kind: TypeKind::Struct,
                            generics: Vec::new(),
                        })
                    }
                    word::TypeKeyword::Rename => todo!(),
                    word::TypeKeyword::Enum => todo!(),
                    word::TypeKeyword::Props => todo!(),
                },
                Keyword::Use | Keyword::Mod => todo!(),
                Keyword::Stmt(kw) => atom::parse_stmt(
                    self.env.value(),
                    self.scope_proxy(),
                    Some((kw, tokens[0].range.clone())),
                    &tokens[1..],
                ),
                Keyword::Config(cfg) => match cfg {
                    ConfigKeyword::Dataset => {
                        self.env.set_value(Env::DatasetConfig);
                        Ok(Ast::DatasetConfig)
                    }
                },
            }
        } else {
            if tokens.len() >= 2 && tokens[1].kind == TokenKind::Special(Special::Colon) {
                if tokens.len() == 2 {
                    todo!()
                }
                let ident = match tokens[0].kind {
                    TokenKind::Identifier(ident) => match ident {
                        Identifier::Builtin(_) => ast_err!(
                            tokens[0].text_range(),
                            "expect custom identifier but got builtin"
                        )?,
                        Identifier::Custom(custom_ident) => custom_ident,
                    },
                    _ => ast_err!(tokens[0].text_range(), "expect custom identifier")?,
                };
                let ty = atom::parse_ty(self.scope_proxy(), &tokens[2..])?;
                Ok(Ast::MembDef {
                    ident,
                    kind: MembKind::MembVar {
                        ty: InputType {
                            contract: InputContract::Own,
                            ty,
                        },
                    },
                })
            } else {
                atom::parse_stmt(self.env.value(), self.scope_proxy(), None, tokens)
            }
        }
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<AstResult<Ast>> {
        &mut self.folded_results
    }
}
