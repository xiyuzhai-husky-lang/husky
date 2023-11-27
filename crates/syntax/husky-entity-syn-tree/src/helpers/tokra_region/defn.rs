use husky_defn_ast::{DefnAst, DefnAstArena, DefnAstArenaRef, DefnAstIdx, DefnAstIdxRange};
use husky_token::{TokenDb, TokenIdxRange, TokenSheetData};

use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct DefnTokraRegion {
    #[return_ref]
    _tokens_data: Vec<TokenData>,
    #[return_ref]
    ast_arena: DefnAstArena,
    root_body: DefnAstIdxRange,
    #[return_ref]
    token_group_starts: Vec<RegionalTokenGroupStart>,
    #[return_ref]
    ast_token_idx_ranges: Vec<RegionalTokenIdxRange>,
}

impl DefnTokraRegion {
    pub fn data<'a>(self, db: &'a ::salsa::Db) -> DefnTokraRegionData<'a> {
        DefnTokraRegionData {
            tokens_data: self._tokens_data(db),
            ast_arena: self.ast_arena(db).to_ref(),
            root_body: self.root_body(db),
            ast_token_idx_ranges: self.ast_token_idx_ranges(db),
            token_group_starts: self.token_group_starts(db),
        }
    }

    pub fn tokens_data<'a>(self, db: &'a ::salsa::Db) -> RegionalTokensData<'a> {
        RegionalTokensData::new(self._tokens_data(db))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefnTokraRegionData<'a> {
    tokens_data: &'a [TokenData],
    ast_arena: DefnAstArenaRef<'a>,
    root_body: DefnAstIdxRange,
    token_group_starts: &'a [RegionalTokenGroupStart],
    ast_token_idx_ranges: &'a [RegionalTokenIdxRange],
}

impl<'a> DefnTokraRegionData<'a> {
    #[inline(always)]
    pub fn root_body(self) -> DefnAstIdxRange {
        self.root_body
    }

    #[inline(always)]
    pub fn ast_token_idx_range(self, defn_ast_idx: DefnAstIdx) -> RegionalTokenIdxRange {
        self.ast_token_idx_ranges[defn_ast_idx.index()]
    }

    #[inline(always)]
    pub fn token_stream(
        self,
        regional_token_group_idx: RegionalTokenGroupIdx,
    ) -> RegionalTokenStream<'a> {
        let regional_token_group_start = self.token_group_starts[regional_token_group_idx.index()];
        let start_index = regional_token_group_start.index();
        let end_index = self
            .token_group_starts
            .get(regional_token_group_idx.index() + 1)
            .map(|&end| end.index())
            .unwrap_or(self.tokens_data.len());

        RegionalTokenStream::new_defn_regional_token_stream(
            &self.tokens_data[start_index..end_index],
            regional_token_group_start,
        )
    }

    pub fn ast_arena(self) -> DefnAstArenaRef<'a> {
        self.ast_arena
    }
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DefnTokraRegionData<'a> {
    type Output = TokenData;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens_data[idx.index()]
    }
}

impl<'a> std::ops::Index<RegionalTokenGroupIdx> for DefnTokraRegionData<'a> {
    type Output = [TokenData];

    fn index(&self, regional_token_group_idx: RegionalTokenGroupIdx) -> &Self::Output {
        let start = self.token_group_starts[regional_token_group_idx.index()].index();
        let end = self
            .token_group_starts
            .get(regional_token_group_idx.index() + 1)
            .map(|&end| end.index())
            .unwrap_or(self.tokens_data.len());
        &self.tokens_data[start..end]
    }
}

impl<'a> std::ops::Index<DefnAstIdx> for DefnTokraRegionData<'a> {
    type Output = DefnAst;

    fn index(&self, idx: DefnAstIdx) -> &Self::Output {
        &self.ast_arena[idx]
    }
}

pub(super) fn defn_token_region(
    syn_node_path: ItemSynNodePath,
    _db: &::salsa::Db,
) -> Option<DefnTokraRegion> {
    match syn_node_path {
        ItemSynNodePath::Submodule(_, _) => None,
        ItemSynNodePath::MajorItem(_) => todo!(),
        ItemSynNodePath::TypeVariant(_, _) => todo!(),
        ItemSynNodePath::ImplBlock(_) => todo!(),
        ItemSynNodePath::AssociatedItem(_) => todo!(),
        ItemSynNodePath::Attr(_, _) => todo!(),
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct SynDefnTokraRegionSourceMap {
    pub regional_token_group_idx_base: RegionalTokenGroupIdxBase,
    pub regional_token_idx_base: RegionalTokenIdxBase,
    #[return_ref]
    pub ast_idx_map: Vec<AstIdx>,
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn item_syn_defn_tokra_region_with_source_map(
    db: &::salsa::Db,
    id: ItemSynNodePathId,
) -> Option<()> {
    let syn_node = id.syn_node(db);
    let builder =
        SynDefnTokraRegionBuilder::new(syn_node.module_path(db), syn_node.ast_idx(db), db)?;
    Some(builder.build())
}

struct SynDefnTokraRegionBuilder<'a> {
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
    defn_ast_arena: DefnAstArena,
    root_body: AstIdxRange,
    regional_token_group_idx_base: RegionalTokenGroupIdxBase,
    regional_token_idx_base: RegionalTokenIdxBase,
    token_sheet_data: &'a TokenSheetData,
    ast_idx_map: Vec<AstIdx>,
    regional_token_idx_range_map: Vec<RegionalTokenIdxRange>,
    tokens_data: Vec<TokenData>,
    db: &'a ::salsa::Db,
}

impl<'a> SynDefnTokraRegionBuilder<'a> {
    fn new(module_path: ModulePath, ast_idx: AstIdx, db: &'a ::salsa::Db) -> Option<Self> {
        // let
        let ast_sheet = module_path.ast_sheet(db);
        let root_body = match ast_sheet[ast_idx] {
            Ast::Identifiable { block, .. } => block.children()?,
            _ => unreachable!(),
        };
        let Some((first_ast_idx, first_token_group_idx)) =
            root_body
                .into_iter()
                .find_map(|ast_idx| match ast_sheet[ast_idx] {
                    Ast::Err {
                        token_group_idx, ..
                    }
                    | Ast::BasicStmtOrBranch {
                        token_group_idx, ..
                    }
                    | Ast::MatchStmt {
                        token_group_idx, ..
                    } => Some((ast_idx, token_group_idx)),
                    Ast::IfElseStmts { if_branch, .. } => Some((
                        ast_idx,
                        match ast_sheet[if_branch] {
                            Ast::BasicStmtOrBranch {
                                token_group_idx, ..
                            } => token_group_idx,
                            _ => unreachable!(),
                        },
                    )),
                    _ => None,
                })
        else {
            todo!()
        };
        let regional_token_group_idx_base =
            RegionalTokenGroupIdxBase::from_token_group_idx(first_token_group_idx);
        let ast_token_idx_range_sheet = module_path.ast_token_idx_range_sheet(db);
        let token_sheet_data = db.token_sheet_data(module_path);
        let token_idx_range: TokenIdxRange = ast_token_idx_range_sheet[first_ast_idx]
            .join(ast_token_idx_range_sheet[root_body.end() - 1]);
        let tokens_data = token_sheet_data[token_idx_range].to_vec();
        let regional_token_idx_base =
            RegionalTokenIdxBase::new(token_sheet_data.token_group_start(first_token_group_idx));
        Some(Self {
            db,
            ast_sheet,
            defn_ast_arena: Default::default(),
            root_body,
            regional_token_group_idx_base,
            tokens_data,
            regional_token_idx_base,
            token_sheet_data,
            ast_token_idx_range_sheet,
            ast_idx_map: Default::default(),
            regional_token_idx_range_map: Default::default(),
        })
    }

    fn build(mut self) -> (DefnTokraRegion, SynDefnTokraRegionSourceMap) {
        let root_body = self.build_asts(self.root_body);
        self.finish(root_body)
    }

    fn build_asts(&mut self, ast_idx_range: AstIdxRange) -> DefnAstIdxRange {
        let mut ast_idxs = vec![];
        let mut regional_token_idx_ranges = vec![];
        let mut regional_asts = vec![];
        for ast_idx in ast_idx_range {
            if let Some(regional_ast) = self.build_ast(ast_idx) {
                ast_idxs.push(ast_idx);
                regional_token_idx_ranges.push(RegionalTokenIdxRange::from_token_idx_range(
                    self.ast_token_idx_range_sheet[ast_idx],
                    self.regional_token_idx_base,
                ));
                regional_asts.push(regional_ast)
            }
        }
        let regional_ast_idx_range = self.defn_ast_arena.alloc_batch(regional_asts);
        debug_assert_eq!(
            regional_ast_idx_range.start().index(),
            self.ast_idx_map.len()
        );
        debug_assert_eq!(
            regional_ast_idx_range.start().index(),
            self.regional_token_idx_range_map.len()
        );
        self.ast_idx_map.extend(ast_idxs);
        self.regional_token_idx_range_map
            .extend(regional_token_idx_ranges);
        regional_ast_idx_range
    }

    fn build_ast(&mut self, ast_idx: AstIdx) -> Option<DefnAst> {
        match self.ast_sheet[ast_idx] {
            Ast::Err {
                token_group_idx: _,
                error: _,
            } => Some(DefnAst::Err),
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
            } => Some(DefnAst::BasicStmtOrBranch {
                regional_token_group_idx: RegionalTokenGroupIdx::new(
                    token_group_idx,
                    self.regional_token_group_idx_base,
                ),
                body: body.map(|body| self.build_asts(body.ast_idx_range())),
            }),
            Ast::IfElseStmts {
                if_branch,
                elif_branches,
                else_branch,
            } => Some(DefnAst::IfElseStmts {
                if_branch: self.build_ast_then_alloc(if_branch).expect("todo"),
                elif_branches: self.build_asts(elif_branches),
                else_branch: else_branch
                    .map(|else_branch| self.build_ast_then_alloc(else_branch).expect("todo")),
            }),
            Ast::MatchStmt {
                token_group_idx,
                pattern_stmt,
                case_branches,
            } => Some(DefnAst::MatchStmt {
                regional_token_group_idx: RegionalTokenGroupIdx::new(
                    token_group_idx,
                    self.regional_token_group_idx_base,
                ),
                pattern_stmt: self.build_ast_then_alloc(pattern_stmt).expect("todo"),
                case_branches: self.build_asts(case_branches),
            }),
            _ => None,
        }
    }

    fn build_ast_then_alloc(&mut self, ast_idx: AstIdx) -> Option<DefnAstIdx> {
        let regional_ast = self.build_ast(ast_idx)?;
        let regional_ast_idx = self.defn_ast_arena.alloc_one(regional_ast);
        self.ast_idx_map.push(ast_idx);
        self.regional_token_idx_range_map
            .push(RegionalTokenIdxRange::from_token_idx_range(
                self.ast_token_idx_range_sheet[ast_idx],
                self.regional_token_idx_base,
            ));
        Some(regional_ast_idx)
    }

    fn finish(self, root_body: DefnAstIdxRange) -> (DefnTokraRegion, SynDefnTokraRegionSourceMap) {
        let regional_token_group_starts = (self.regional_token_group_idx_base.index()..)
            .into_iter()
            .map_while(|token_group_index| {
                let token_group_start = *self
                    .token_sheet_data
                    .token_group_starts()
                    .get(token_group_index)?;
                if self.regional_token_idx_base.index_base() + self.tokens_data.len()
                    <= token_group_start.index()
                {
                    return None;
                }
                Some(RegionalTokenGroupStart::from_token_group_start(
                    token_group_start,
                    self.regional_token_idx_base,
                ))
            })
            .collect();
        (
            DefnTokraRegion::new(
                self.db,
                self.tokens_data,
                self.defn_ast_arena,
                root_body,
                regional_token_group_starts,
                self.regional_token_idx_range_map,
            ),
            SynDefnTokraRegionSourceMap::new(
                self.db,
                self.regional_token_group_idx_base,
                self.regional_token_idx_base,
                self.ast_idx_map,
            ),
        )
    }
}

fn build_defn_tokra_region(
    module_path: ModulePath,
    ast_idx: AstIdx,
    db: &::salsa::Db,
) -> Option<(DefnTokraRegion, SynDefnTokraRegionSourceMap)> {
    let builder = SynDefnTokraRegionBuilder::new(module_path, db, ast_idx)?;
    Some(builder.build())
}

impl ItemSynNodePath {
    pub fn defn_tokra_region(self, db: &::salsa::Db) -> Option<DefnTokraRegion> {
        todo!()
    }

    // use this only when necessary
    pub fn defn_tokra_region_source_map(
        self,
        db: &::salsa::Db,
    ) -> Option<SynDefnTokraRegionSourceMap> {
        todo!()
    }

    pub fn defn_regional_token_idx_base(self, db: &::salsa::Db) -> Option<RegionalTokenIdxBase> {
        Some(
            self.defn_tokra_region_source_map(db)?
                .regional_token_idx_base(db),
        )
    }
}
