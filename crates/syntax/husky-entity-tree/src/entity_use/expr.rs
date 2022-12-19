use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr, Bracket};
use husky_print_utils::p;
use husky_token::{Keyword, SpecialToken, TokenIdx, TokenIter, TokenKind, TokenSheet};
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq)]
pub enum EntityUseExpr {
    // *
    All {},
    One {
        ident: Identifier,
    },
    ScopeResolution {
        parent: Identifier,
        child: EntityUseExprIdx,
    },
    Multiple {
        exprs: EntityUseExprIdxRange,
    },
    Err(EntityUseExprError),
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum EntityUseExprError {
    #[error("expect something")]
    ExpectSomething,
    #[error("expect identifier or `{{` or `*`")]
    ExpectIdentifierOrLCurlyOrStar,
    #[error("expect `::`")]
    ExpectScopeResolution,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityUseExprSheet {
    arena: EntityUseExprArena,
    use_exprs: Vec<(AstIdx, Accessibility, EntityUseExprIdx)>,
}

impl std::ops::Deref for EntityUseExprSheet {
    type Target = EntityUseExprArena;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl EntityUseExprSheet {
    pub fn use_exprs(&self) -> &[(ArenaIdx<Ast>, Accessibility, ArenaIdx<EntityUseExpr>)] {
        self.use_exprs.as_ref()
    }
}

pub type EntityUseExprArena = Arena<EntityUseExpr>;
pub type EntityUseExprIdx = ArenaIdx<EntityUseExpr>;
pub type EntityUseExprIdxRange = ArenaIdxRange<EntityUseExpr>;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_use_exprs(
    db: &dyn EntityTreeDb,
    module_path: EntityPath,
) -> VfsResult<EntityUseExprSheet> {
    let ast_sheet = db.ast_sheet(module_path).as_ref()?;
    let token_sheet = db.token_sheet(module_path).as_ref()?;
    Ok(EntityUseExprCollector::new(db, ast_sheet, token_sheet).collect_all())
}

#[test]
fn moule_use_exprs_works() {
    DB::expect_test_modules("module_use_sheet", |db, module_path| {
        module_use_exprs(db, module_path)
    })
}

pub struct EntityUseExprCollector<'a> {
    db: &'a dyn EntityTreeDb,
    ast_sheet: &'a AstSheet,
    token_sheet: &'a TokenSheet,
    arena: EntityUseExprArena,
}

impl<'a> EntityUseExprCollector<'a> {
    fn new(db: &'a dyn EntityTreeDb, ast_sheet: &'a AstSheet, token_sheet: &'a TokenSheet) -> Self {
        Self {
            db,
            ast_sheet,
            token_sheet,
            arena: Default::default(),
        }
    }

    fn collect_all(mut self) -> EntityUseExprSheet {
        EntityUseExprSheet {
            // order matters!
            use_exprs: self
                .ast_sheet
                .indexed_asts()
                .filter_map(|(ast_idx, ast)| {
                    self.collect_from_ast(ast)
                        .map(|(accessibility, expr)| (ast_idx, accessibility, expr))
                })
                .collect(),
            arena: self.arena,
        }
    }

    fn collect_from_ast(&mut self, ast: &Ast) -> Option<(Accessibility, EntityUseExprIdx)> {
        match ast {
            Ast::Use {
                token_group_idx,
                accessibility,
            } => {
                let mut token_iter = self.token_sheet.token_group_token_iter(*token_group_idx);
                Some((
                    *accessibility,
                    match EntityUseExprParser::new(self.db, token_iter, &mut self.arena)
                        .parse_step()
                    {
                        Some(expr) => expr,
                        None => self.arena.alloc_one(EntityUseExpr::Err(todo!())),
                    },
                ))
            }
            _ => None,
        }
    }
}

pub struct EntityUseExprParser<'b> {
    db: &'b dyn EntityTreeDb,
    token_iter: TokenIter<'b>,
    arena: &'b mut EntityUseExprArena,
}

impl<'b> EntityUseExprParser<'b> {
    fn new(
        db: &'b dyn EntityTreeDb,
        mut token_iter: TokenIter<'b>,
        arena: &'b mut EntityUseExprArena,
    ) -> Self {
        while let Some(token) = token_iter.next() {
            match token.kind {
                TokenKind::Keyword(Keyword::Use) => break,
                _ => continue,
            }
        }
        Self {
            db,
            token_iter,
            arena,
        }
    }

    fn parse_step(&mut self) -> Option<EntityUseExprIdx> {
        let expr = self.parse_step_inner()?;
        Some(self.arena.alloc_one(expr))
    }

    fn parse_step_inner(&mut self) -> Option<EntityUseExpr> {
        let token = self.token_iter.next()?;
        Some(match token.kind {
            TokenKind::Identifier(ident) => {
                let Some(next_token) = self.token_iter.next() else  {
                    return Some(EntityUseExpr::One { ident })
                };
                match next_token.kind {
                    TokenKind::Special(SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)) => {}
                    TokenKind::Special(SpecialToken::Comma)
                    | TokenKind::Special(SpecialToken::Ket(Bracket::Curl)) => {
                        self.token_iter.step_back();
                        return Some(EntityUseExpr::One { ident });
                    }
                    _ => {
                        return Some(EntityUseExpr::Err(
                            EntityUseExprError::ExpectScopeResolution,
                        ))
                    }
                }
                let Some(child) =  self.parse_step() else {
                    return Some(EntityUseExpr::Err(
                        EntityUseExprError::ExpectScopeResolution,
                    ))
                };
                EntityUseExpr::ScopeResolution {
                    parent: ident,
                    child,
                }
            }
            TokenKind::Special(SpecialToken::Bra(Bracket::Curl)) => self.parse_multiple(),
            // ad hoc; todo: change this to SpecialToken::Star
            TokenKind::Special(SpecialToken::BinaryOpr(BinaryOpr::PureClosed(
                BinaryPureClosedOpr::Mul,
            ))) => EntityUseExpr::All {},
            _ => EntityUseExpr::Err(EntityUseExprError::ExpectSomething),
        })
    }

    fn parse_multiple(&mut self) -> EntityUseExpr {
        let mut exprs: Vec<EntityUseExpr> = vec![];
        loop {
            match self.parse_step_inner() {
                Some(expr) => exprs.push(expr),
                None => todo!(),
            }
            match self.token_iter.next() {
                Some(token) => match token.kind {
                    TokenKind::Special(SpecialToken::Comma) => continue,
                    TokenKind::Special(SpecialToken::Ket(Bracket::Curl)) => break,
                    _ => todo!(),
                },
                None => todo!(),
            }
        }
        EntityUseExpr::Multiple {
            exprs: self.arena.alloc_batch(exprs),
        }
    }
}
