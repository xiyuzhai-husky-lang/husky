use crate::*;
use husky_entity_tree::region_path::SynNodeRegionPath;
use husky_vfs::toolchain::Toolchain;
use vec_like::VecPairMap;

#[salsa::tracked]
pub struct SynExprRegion {
    #[return_ref]
    pub data: SynExprRegionData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SynExprRegionData {
    parent: Option<SynExprRegion>,
    path: SynNodeRegionPath,
    expr_arena: SynExprArena,
    principal_item_path_expr_arena: SynPrincipalEntityPathExprArena,
    stmt_arena: SynStmtArena,
    pattern_expr_region: SynPatternRegion,
    variable_region: VariableRegionData,
    pattern_roots: Vec<SynPatternRoot>,
    expr_roots: Vec<SynExprRoot>,
    has_self_lifetime: bool,
    has_self_place: bool,
    pattern_to_current_variable_map: VecPairMap<PatternVariableIdx, CurrentVariableIdx>,
}

impl SynExprRegionData {
    pub(crate) fn new(
        parent: Option<SynExprRegion>,
        path: SynNodeRegionPath,
        expr_arena: SynExprArena,
        principal_item_path_expr_arena: SynPrincipalEntityPathExprArena,
        stmt_arena: SynStmtArena,
        pattern_expr_region: SynPatternRegion,
        variable_region: VariableRegionData,
        pattern_roots: Vec<SynPatternRoot>,
        expr_roots: Vec<SynExprRoot>,
        has_self_lifetime: bool,
        has_self_place: bool,
    ) -> Self {
        let pattern_to_current_variable_map = VecPairMap::from_iter_assuming_no_repetitions(
            variable_region
                .current_variable_arena()
                .indexed_iter()
                .filter_map(|(current_variable_idx, current_variable)| {
                    match *current_variable.data() {
                        CurrentVariableData::SimpleParenateParameter {
                            pattern_variable_idx,
                            ..
                        }
                        | CurrentVariableData::LetVariable {
                            pattern_variable_idx,
                            ..
                        }
                        | CurrentVariableData::BeVariable {
                            pattern_variable_idx,
                            ..
                        }
                        | CurrentVariableData::CaseVariable {
                            pattern_variable_idx,
                            ..
                        } => Some((pattern_variable_idx, current_variable_idx)),
                        _ => None,
                    }
                }),
        )
        .expect("no repetition");
        Self {
            parent,
            path,
            expr_arena,
            principal_item_path_expr_arena,
            stmt_arena,
            pattern_expr_region,
            variable_region,
            pattern_roots,
            expr_roots,
            has_self_lifetime,
            has_self_place,
            pattern_to_current_variable_map,
        }
    }

    pub fn parent(&self) -> Option<SynExprRegion> {
        self.parent
    }

    pub fn path(&self) -> SynNodeRegionPath {
        self.path
    }

    pub fn path_ref(&self) -> &SynNodeRegionPath {
        &self.path
    }

    pub fn expr_arena(&self) -> &SynExprArena {
        &self.expr_arena
    }

    pub fn principal_item_path_expr_arena(&self) -> &SynPrincipalEntityPathExprArena {
        &self.principal_item_path_expr_arena
    }

    pub fn pattern_expr_arena(&self) -> &SynPatternArena {
        self.pattern_expr_region.pattern_expr_arena()
    }

    pub fn stmt_arena(&self) -> &SynStmtArena {
        &self.stmt_arena
    }

    pub fn pattern_expr_region(&self) -> &SynPatternRegion {
        &self.pattern_expr_region
    }

    pub fn variable_region(&self) -> &VariableRegionData {
        &self.variable_region
    }

    pub fn syn_expr_roots(&self) -> &[SynExprRoot] {
        self.expr_roots.as_ref()
    }

    pub fn return_ty(&self) -> Option<SynExprIdx> {
        self.expr_roots.iter().find_map(|root| {
            (root.kind() == SynExprRootKind::ReturnType).then_some(root.syn_expr_idx())
        })
    }

    pub fn syn_pattern_expr_roots(&self) -> &[SynPatternRoot] {
        self.pattern_roots.as_ref()
    }

    pub fn self_ty(&self) -> Option<SynExprIdx> {
        todo!();
        // ad hoc
        // this will not work for derive any decl region\
        // self.roots
        //     .iter()
        //     .find_map(|root| (root.kind() == ExprRootKind::SelfType).then_some(root.expr_idx()))
    }

    pub fn syn_pattern_to_current_variable(
        &self,
        syn_pattern_variable_idx: PatternVariableIdx,
    ) -> CurrentVariableIdx {
        self.pattern_to_current_variable_map[syn_pattern_variable_idx].1
    }

    pub fn syn_pattern_expr_current_variables_mapped<R>(
        &self,
        syn_pattern_idx: SynPatternIdx,
        f: impl Fn(CurrentVariableIdx) -> R,
    ) -> IdentPairMap<R> {
        unsafe {
            IdentPairMap::from_iter_assuming_no_repetitions_unchecked(
                self.pattern_expr_region()
                    .pattern_expr_symbols(syn_pattern_idx)
                    .iter()
                    .map(|&(ident, syn_pattern_variable_idx)| {
                        let current_variable_idx =
                            self.syn_pattern_to_current_variable(syn_pattern_variable_idx);
                        (ident, f(current_variable_idx))
                    }),
            )
        }
    }

    pub fn has_self_lifetime(&self) -> bool {
        self.has_self_lifetime
    }

    pub fn has_self_place(&self) -> bool {
        self.has_self_place
    }
}

impl std::ops::Index<SynExprIdx> for SynExprRegionData {
    type Output = SynExprData;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.expr_arena[index]
    }
}

impl std::ops::Index<SynStmtIdx> for SynExprRegionData {
    type Output = SynStmtData;

    fn index(&self, index: SynStmtIdx) -> &Self::Output {
        &self.stmt_arena[index]
    }
}
impl std::ops::Index<CurrentVariableIdx> for SynExprRegionData {
    type Output = CurrentVariableEntry;

    fn index(&self, index: CurrentVariableIdx) -> &Self::Output {
        &self.variable_region[index]
    }
}
impl std::ops::Index<PatternVariableIdx> for SynExprRegionData {
    type Output = PatternVariable;

    fn index(&self, index: PatternVariableIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}
impl std::ops::Index<SynPatternIdx> for SynExprRegionData {
    type Output = SynPatternData;

    fn index(&self, index: SynPatternIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}

impl SynExprRegion {
    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        match self.data(db).path {
            SynNodeRegionPath::CrateDecl(slf) => slf.toolchain(db),
            SynNodeRegionPath::ItemDecl(slf) | SynNodeRegionPath::ItemDefn(slf) => {
                slf.toolchain(db)
            }
        }
    }
}
