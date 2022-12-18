use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_token::{TokenIdx, TokenStream};
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq)]
pub enum EntityUseExpr {
    // *
    All {
        start_token_idx: TokenIdx,
    },
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
pub enum EntityUseExprError {}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityUseExprSheet {
    arena: EntityUseExprArena,
    use_exprs: Vec<(AstIdx, EntityUseExprIdx)>,
}

pub type EntityUseExprArena = Arena<EntityUseExpr>;
pub type EntityUseExprIdx = ArenaIdx<EntityUseExpr>;
pub type EntityUseExprIdxRange = ArenaIdxRange<EntityUseExpr>;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_use_exprs(
    db: &dyn EntityTreeDb,
    module_path: EntityPath,
) -> EntityUseExprSheet {
    todo!()
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
    arena: EntityUseExprArena,
}

impl<'a> EntityUseExprCollector<'a> {
    fn collect_all(mut self) -> EntityUseExprSheet {
        let exprs: Vec<(AstIdx, EntityUseExprIdx)> = self
            .ast_sheet
            .indexed_asts()
            .filter_map(|(ast_idx, ast)| self.collect_from_ast(ast).map(|expr| (ast_idx, expr)))
            .collect();
        todo!()
    }

    fn collect_from_ast(&mut self, ast: &Ast) -> Option<EntityUseExprIdx> {
        let parser = EntityUseExprParser {
            db: self.db,
            token_iter: todo!(),
        };
        todo!()
    }
}

pub struct EntityUseExprParser<'a> {
    db: &'a dyn EntityTreeDb,
    token_iter: TokenStream<'a>,
}

impl<'a> EntityUseExprParser<'a> {
    fn parse(mut self) -> EntityTreeResult<EntityUseExprIdx> {
        todo!()
    }
}
