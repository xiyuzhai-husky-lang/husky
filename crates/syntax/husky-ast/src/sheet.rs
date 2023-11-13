use crate::*;
use husky_vfs::error::VfsResult;

pub trait HasAstSheet: Copy {
    fn ast_sheet(self, db: &dyn AstDb) -> VfsResult<&AstSheet>;
    fn ast_token_idx_range_sheet(self, db: &dyn AstDb) -> VfsResult<&AstTokenIdxRangeSheet>;
}

impl HasAstSheet for ModulePath {
    fn ast_sheet(self, db: &dyn AstDb) -> VfsResult<&AstSheet> {
        Ok(ast_sheet(db, self).as_ref()?)
    }

    fn ast_token_idx_range_sheet(self, db: &dyn AstDb) -> VfsResult<&AstTokenIdxRangeSheet> {
        Ok(ast_token_idx_range_sheet(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_sheet(db: &dyn AstDb, module_path: ModulePath) -> VfsResult<AstSheet> {
    Ok(AstParser::new(db, module_path)?.parse_all())
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub struct AstSheet {
    ast_arena: AstArena,
    top_level_asts: AstIdxRange,
    // a list of siblings indices
    // list index has nothing to do with ast idx
    siblings: Vec<AstIdxRange>,
}

impl std::ops::Index<AstIdx> for AstSheet {
    type Output = Ast;

    fn index(&self, index: AstIdx) -> &Self::Output {
        &self.ast_arena[index]
    }
}

impl AstSheet {
    pub(crate) fn new(
        ast_arena: AstArena,
        top_level_asts: AstIdxRange,
        siblings: Vec<AstIdxRange>,
    ) -> Self {
        Self {
            ast_arena,
            top_level_asts,
            siblings,
        }
    }

    pub fn all_ast_indexed_iter<'a>(&'a self) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.ast_arena.indexed_iter()
    }

    pub fn indexed_iter<'a>(
        &'a self,
        ast_idx_range: AstIdxRange,
    ) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        ast_idx_range.into_iter().map(|idx| (idx, &self[idx]))
    }

    pub fn top_level_asts(&self) -> &AstIdxRange {
        &self.top_level_asts
    }

    pub fn top_level_asts_iter<'a>(&'a self) -> impl Iterator<Item = &'a Ast> + 'a {
        self.ast_arena[&self.top_level_asts].iter()
    }

    pub fn top_level_asts_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (AstIdx, &'a Ast)> + 'a {
        self.ast_arena[&self.top_level_asts]
            .iter()
            .enumerate()
            .map(|(i, ast)| (self.top_level_asts.start() + i, ast))
    }

    pub fn siblings(&self) -> &[AstIdxRange] {
        &self.siblings
    }
}

impl std::ops::Deref for AstSheet {
    type Target = AstArena;

    fn deref(&self) -> &Self::Target {
        &self.ast_arena
    }
}
