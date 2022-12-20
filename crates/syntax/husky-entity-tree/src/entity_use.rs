use crate::*;

use husky_ast::{Ast, AstSheet};
use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr, Bracket};

use husky_token::{Keyword, SpecialToken, TokenIter, TokenKind, TokenSheet};
use husky_word::Identifier;
use idx_arena::map::ArenaMap;


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

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityUseExpr {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::All {} => f.debug_struct("All").finish(),
            Self::One { ident } => f
                .debug_struct("One")
                .field("ident", &ident.debug_with(db, include_all_fields))
                .finish(),
            Self::ScopeResolution { parent, child } => f
                .debug_struct("ScopeResolution")
                .field("parent", &parent.debug_with(db, include_all_fields))
                .field("child", child)
                .finish(),
            Self::Multiple { exprs } => f.debug_struct("Multiple").field("exprs", exprs).finish(),
            Self::Err(arg0) => f.debug_tuple("Err").field(arg0).finish(),
        }
    }
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
pub(crate) struct EntityUseExprSheet {
    arena: EntityUseExprArena,
    use_expr_roots: Vec<(Accessibility, Identifier, EntityUseExprIdx, AstIdx)>,
}

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityUseExprSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("EntityUseExprSheet")
            .field("arena", &self.arena.debug_with(db, include_all_fields))
            .field("use_expr_roots", &self.use_expr_roots)
            .finish()
    }
}

pub(crate) struct EntityUseExprResolveSheet<'a> {
    sheet: &'a EntityUseExprSheet,
    roots: ArenaMap<EntityUseExpr, EntityPath>,
    modified: Vec<EntityUseExprIdx>,
}

impl<'a> EntityUseExprResolveSheet<'a> {
    pub fn next(&mut self) -> Option<&'a EntityUseExprVisitor> {
        todo!()
    }

    pub fn peek(&mut self) -> Option<&'a EntityUseExprVisitor> {
        todo!()
    }
}

pub struct EntityUseExprVisitor<'a> {
    arena: &'a EntityUseExprArena,
    expr: &'a EntityUseExpr,
}

impl std::ops::Deref for EntityUseExprSheet {
    type Target = EntityUseExprArena;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl EntityUseExprSheet {
    pub fn use_exprs(
        &self,
    ) -> &[(
        Accessibility,
        Identifier,
        ArenaIdx<EntityUseExpr>,
        ArenaIdx<Ast>,
    )] {
        self.use_expr_roots.as_ref()
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
    DB::expect_test_probable_modules_debug_with_db("module_use_sheet", |db, module_path| {
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
            use_expr_roots: self
                .ast_sheet
                .indexed_asts()
                .filter_map(|(ast_idx, ast)| {
                    self.collect_from_ast(ast)
                        .map(|(accessibility, ident, expr)| (accessibility, ident, expr, ast_idx))
                })
                .collect(),
            arena: self.arena,
        }
    }

    fn collect_from_ast(
        &mut self,
        ast: &Ast,
    ) -> Option<(Accessibility, Identifier, EntityUseExprIdx)> {
        match ast {
            Ast::Use {
                token_group_idx,
                accessibility,
            } => {
                let token_iter = self.token_sheet.token_group_token_iter(*token_group_idx);
                let (ident, mut entity_use_expr_parser) =
                    EntityUseExprParser::new(self.db, token_iter, &mut self.arena);
                Some((
                    *accessibility,
                    ident,
                    match entity_use_expr_parser.parse_step() {
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
    ) -> (Identifier, Self) {
        while let Some(token) = token_iter.next() {
            match token.kind {
                TokenKind::Keyword(Keyword::Use) => break,
                _ => continue,
            }
        }
        let ident = match token_iter.peek() {
            Some(token) => match token.kind {
                TokenKind::Decorator(_) => todo!(),
                TokenKind::Keyword(_) => todo!(),
                TokenKind::Identifier(ident) => ident,
                TokenKind::Special(_) => todo!(),
                TokenKind::WordOpr(_) => todo!(),
                TokenKind::Literal(_) => todo!(),
                TokenKind::Unrecognized(_) => todo!(),
                TokenKind::IllFormedLiteral(_) => todo!(),
                TokenKind::Comment => todo!(),
            },
            None => todo!(),
        };
        (
            ident,
            Self {
                db,
                token_iter,
                arena,
            },
        )
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityUseAtom {
    All {
        ast_idx: AstIdx,
        parent: Vec<Identifier>,
        accessibility: Accessibility,
    },
    One {
        ast_idx: AstIdx,
        path: Vec<Identifier>,
        accessibility: Accessibility,
    },
}

pub struct UseAllCollector<'a> {
    db: &'a dyn EntityTreeDb,
    module_use_expr_sheet: &'a EntityUseExprSheet,
    path: Vec<Identifier>,
    use_atoms: &'a mut Vec<EntityUseAtom>,
    ast_idx: AstIdx,
    root: EntityUseExprIdx,
    accessibility: Accessibility,
}

impl<'a> UseAllCollector<'a> {
    fn new(
        db: &'a dyn EntityTreeDb,
        module_use_expr_sheet: &'a EntityUseExprSheet,
        ast_idx: AstIdx,
        root: EntityUseExprIdx,
        accessibility: Accessibility,
        use_atoms: &'a mut Vec<EntityUseAtom>,
    ) -> Self {
        Self {
            db,
            module_use_expr_sheet,
            use_atoms,
            ast_idx,
            root,
            path: vec![],
            accessibility,
        }
    }
    fn collect_all(mut self) {
        self.collect(self.root);
    }

    fn collect(&mut self, use_expr_idx: EntityUseExprIdx) {
        match &self.module_use_expr_sheet[use_expr_idx] {
            EntityUseExpr::All {} => self.use_atoms.push(EntityUseAtom::All {
                ast_idx: self.ast_idx,
                parent: self.path.clone(),
                accessibility: self.accessibility,
            }),
            EntityUseExpr::One { ident } => {
                let mut path = self.path.clone();
                path.push(*ident);
                self.use_atoms.push(EntityUseAtom::One {
                    ast_idx: self.ast_idx,
                    path,
                    accessibility: self.accessibility,
                })
            }
            EntityUseExpr::ScopeResolution { parent, child } => {
                self.path.push(*parent);
                self.collect(*child)
            }
            EntityUseExpr::Multiple { exprs: _ } => todo!(),
            EntityUseExpr::Err(_) => todo!(),
        }
    }
}
