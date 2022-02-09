mod impl_parse_expr;
mod impl_parse_func_decl;
mod impl_parse_stmt;
mod impl_symbol_proxy;
mod impl_use_all;
mod utils;

use file::FilePtr;
use fold::{FoldedList, LocalStack, LocalValue};
use scope::ScopeRoute;
use scope_query::PackageOrModule;
use syntax_types::*;
use text::TextRanged;
use token::*;
use vm::InputContract;
use word::*;

use crate::{
    atom::symbol_proxy::Symbol,
    query::{AstQueryGroup, AstText},
    transform::utils::*,
    *,
};

pub struct AstTransformer<'a> {
    db: &'a dyn AstQueryGroup,
    main: FilePtr,
    arena: RawExprArena,
    folded_results: FoldedList<AstResult<Ast>>,
    symbols: LocalStack<Symbol>,
    env: LocalValue<syntax_types::Env>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(db: &'a dyn AstQueryGroup, module: PackageOrModule) -> Self {
        Self {
            db,
            main: db
                .main_file_id(db.module_to_file_id(module).unwrap())
                .unwrap(),
            arena: RawExprArena::new(),
            folded_results: FoldedList::new(),
            symbols: LocalStack::new(),
            env: LocalValue::new(match module.scope().route {
                ScopeRoute::Package { .. } => Env::Package,
                ScopeRoute::ChildScope { .. } => Env::Module,
                ScopeRoute::Builtin { .. } | ScopeRoute::Implicit { .. } => panic!(),
            }),
        }
    }

    pub(crate) fn finish(self) -> AstText {
        AstText {
            arena: self.arena,
            folded_results: self.folded_results,
        }
    }

    fn env(&self) -> Env {
        self.env.value()
    }
}

impl<'a> fold::Transformer<[Token], TokenizedText, AstResult<Ast>> for AstTransformer<'a> {
    fn _enter_block(&mut self) {
        self.env.enter();
        self.symbols.enter();
    }

    fn _exit_block(&mut self) {
        self.env.exit();
        self.symbols.exit();
    }

    fn transform(
        &mut self,
        _indent: fold::Indent,
        tokens: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<Ast> {
        if let TokenKind::Keyword(keyword) = tokens[0].kind {
            match keyword {
                Keyword::Func(func_kw) => match func_kw {
                    word::FuncKeyword::Main => {
                        self.env.set_value(Env::Main);
                        Ok(Ast::MainDef)
                    }
                    word::FuncKeyword::Test => {
                        self.env.set_value(Env::Test);
                        todo!()
                    }
                    word::FuncKeyword::Proc => {
                        self.env.set_value(Env::Proc);
                        todo!()
                    }
                    word::FuncKeyword::Func => Ok(Ast::FuncDef {
                        kind: FuncKind::PureFunc,
                        decl: self.parse_func_decl(trim!(tokens; keyword, colon))?,
                    }),
                    word::FuncKeyword::Def => todo!(),
                },
                Keyword::Type(ty_kw) => match ty_kw {
                    word::TypeKeyword::Struct => {
                        expect_len!(tokens, 3);
                        expect_head!(tokens);
                        Ok(Ast::TypeDef {
                            ident: identify!(tokens[1]),
                            kind: TyKind::Struct,
                            generics: Vec::new(),
                        })
                    }
                    word::TypeKeyword::Rename => todo!(),
                    word::TypeKeyword::Enum => todo!(),
                    word::TypeKeyword::Props => todo!(),
                },
                Keyword::Use | Keyword::Mod => todo!(),
                Keyword::Stmt(kw) => self
                    .parse_stmt(Some((kw, tokens[0].range.clone())), &tokens[1..])
                    .map(|stmt| stmt.into()),
                Keyword::Config(cfg) => match cfg {
                    ConfigKeyword::Dataset => {
                        self.env.set_value(Env::DatasetConfig);
                        enter_block(self);
                        self.use_all(
                            BuiltinIdentifier::DatasetType.into(),
                            tokens[0].text_range(),
                        )?;
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
                        Identifier::Implicit(_) => ast_err!(
                            tokens[0].text_range(),
                            "expect implicit identifier but got builtin"
                        )?,
                        Identifier::Custom(custom_ident) => custom_ident,
                    },
                    _ => ast_err!(tokens[0].text_range(), "expect custom identifier")?,
                };
                let ty = atom::parse_ty(self.symbol_proxy(), &tokens[2..])?;
                Ok(Ast::MembDef {
                    ident,
                    kind: MembKind::MembVar {
                        ty: MembType {
                            contract: InputContract::Own,
                            scope: ty,
                        },
                    },
                })
            } else {
                self.parse_stmt(None, tokens).map(|stmt| stmt.into())
            }
        }
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<AstResult<Ast>> {
        &mut self.folded_results
    }
}
