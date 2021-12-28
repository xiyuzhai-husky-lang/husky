use folded::FoldedList;
use token::{Special, Token, TokenKind, TokenizedText};
use word::CustomIdentifier;

use crate::{
    parser::ScopeLRParser,
    query::AtomQuery,
    scope_proxy::ScopeProxy,
    types::{Contract, ContractedType, MembKind},
    *,
};

pub struct AtomicTransformer<'a> {
    db: &'a dyn AtomQuery,
    folded_results: FoldedList<AtomParseResult>,
    symbols: Vec<(CustomIdentifier, common::Range)>,
}

impl<'a> AtomicTransformer<'a> {
    pub(crate) fn new(db: &'a dyn AtomQuery, module: scope::Module) -> Self {
        Self {
            db,
            folded_results: FoldedList::new(),
            symbols: Vec::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<AtomParseResult> {
        self.folded_results
    }

    fn scope_proxy(&mut self) -> ScopeProxy {
        ScopeProxy {
            db: self.db,
            symbols: &self.symbols,
            line: 0,
        }
    }
}

impl<'a> folded::Transformer<[Token], TokenizedText, AtomParseResult> for AtomicTransformer<'a> {
    fn enter(&mut self) {}

    fn exit(&mut self) {}

    fn post_exit(&mut self, idx: folded::FoldedIdx<AtomParseResult>) {}

    fn transform(&mut self, indent: folded::Indent, tokens: &[Token]) -> AtomParseResult {
        use parser::*;

        if let TokenKind::Keyword(keyword) = tokens[0].kind {
            match keyword {
                Keyword::Func(func_kw) => match func_kw {
                    word::FuncKeyword::Main => Ok(AtomicLineGroup::MainDef),
                    word::FuncKeyword::Test => todo!(),
                    word::FuncKeyword::Proc => todo!(),
                    word::FuncKeyword::Func => {
                        let mut parser =
                            ScopeLRParser::new(self.scope_proxy(), &tokens[1..(tokens.len() - 1)]);
                        let func_decl = parser.func_decl();
                        todo!()
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
                                Identifier::Builtin(_) => atom_err!(
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
                        Ok(AtomicLineGroup::TypeDef {
                            ident,
                            kind: types::TypeKind::Struct,
                            args: Vec::new(),
                        })
                    }
                    word::TypeKeyword::Rename => todo!(),
                    word::TypeKeyword::Enum => todo!(),
                    word::TypeKeyword::Props => todo!(),
                },
                Keyword::Use | Keyword::Mod => todo!(),
                Keyword::Stmt(kw) => {
                    let is_head = tokens.last().unwrap().kind == TokenKind::Special(Special::Colon);
                    let end = if is_head {
                        tokens.len() - 1
                    } else {
                        tokens.len()
                    };
                    AtomicLineGroup::parse_stmt(
                        self.scope_proxy(),
                        Some((kw, tokens[0].range.clone())),
                        is_head,
                        &tokens[1..end],
                    )
                }
            }
        } else {
            if tokens.len() >= 2 && tokens[1].kind == TokenKind::Special(Special::Colon) {
                if tokens.len() == 2 {
                    todo!()
                }
                let ident = match tokens[0].kind {
                    TokenKind::Identifier(ident) => match ident {
                        Identifier::Builtin(_) => atom_err!(
                            tokens[0].text_range(),
                            "expect custom identifier but got builtin"
                        )?,
                        Identifier::Custom(custom_ident) => custom_ident,
                    },
                    _ => atom_err!(tokens[0].text_range(), "expect custom identifier")?,
                };
                let ty = AtomicLineGroup::parse_ty(self.scope_proxy(), &tokens[2..])?;
                Ok(AtomicLineGroup::MembDef {
                    ident,
                    kind: MembKind::MembVar {
                        ty: ContractedType {
                            contract: Contract::Give,
                            ty,
                        },
                    },
                })
            } else {
                dbg!(tokens);
                todo!();
            }
        }
    }

    fn folded_outputs_mut(&mut self) -> &mut FoldedList<AtomParseResult> {
        &mut self.folded_results
    }
}
