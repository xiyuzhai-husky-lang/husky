//! Defines `Body`: a lowered representation of bodies of functions, statics and
//! consts.
mod lower;
pub mod scope;
#[cfg(test)]
mod tests;

use std::{mem, ops::Index, sync::Arc};

use arena::{Arena, ArenaMap};
use base_db::CrateId;
use drop_bomb::DropBomb;
use either::Either;
use hir_expand::{
    ast_id_map::AstIdMap, hygiene::Hygiene, AstId, ExpandResult, HirFileID, InFile, MacroDefId,
};
use limit::Limit;
use profile::Count;
use rustc_hash::FxHashMap;
use syntax::{ast, AstNode, AstPtr, SyntaxNodePtr};

use crate::{
    attr::{Attrs, RawAttrs},
    db::DefDatabase,
    expr::{Expr, ExprId, Label, LabelId, Pat, PatId},
    item_scope::BuiltinShadowMode,
    nameres::DefMap,
    path::{ModPath, Path},
    src::HasSource,
    BlockId, DefWithBodyId, HasModule, LocalModuleId, Lookup, ModuleId,
};

pub use lower::LowerCtx;

#[derive(Debug)]
pub struct Expander {
    def_map: Arc<DefMap>,
    current_file_id: HirFileID,
    ast_id_map: Arc<AstIdMap>,
    module: LocalModuleId,
    recursion_limit: usize,
}

#[cfg(test)]
static EXPANSION_RECURSION_LIMIT: Limit = Limit::new(32);

#[cfg(not(test))]
static EXPANSION_RECURSION_LIMIT: Limit = Limit::new(128);

impl Expander {
    pub fn new(db: &dyn DefDatabase, current_file_id: HirFileID, module: ModuleId) -> Expander {
        todo!()
    }

    pub fn enter_expand<T: ast::AstNode>(
        &mut self,
        db: &dyn DefDatabase,
        macro_call: ast::MacroCall,
    ) -> ExpandResult<Option<(Mark, T)>> {
        todo!()
    }

    pub fn exit(&mut self, db: &dyn DefDatabase, mut mark: Mark) {
        todo!()
    }

    pub(crate) fn to_source<T>(&self, value: T) -> InFile<T> {
        InFile {
            file_id: self.current_file_id,
            value,
        }
    }

    pub(crate) fn parse_attrs(&self, db: &dyn DefDatabase, owner: &dyn ast::HasAttrs) -> Attrs {
        todo!()
    }

    pub fn current_file_id(&self) -> HirFileID {
        self.current_file_id
    }

    fn parse_path(&mut self, db: &dyn DefDatabase, path: ast::Path) -> Option<Path> {
        todo!()
    }

    fn ast_id<N: AstNode>(&self, item: &N) -> AstId<N> {
        let file_local_id = self.ast_id_map.ast_id(item);
        AstId::new(self.current_file_id, file_local_id)
    }
}

#[derive(Debug)]
pub struct Mark {
    file_id: HirFileID,
    ast_id_map: Arc<AstIdMap>,
    bomb: DropBomb,
}

/// The body of an item (function, const etc.).
#[derive(Debug, Eq, PartialEq)]
pub struct Body {
    pub exprs: Arena<Expr>,
    pub pats: Arena<Pat>,
    pub labels: Arena<Label>,
    /// The patterns for the function's parameters. While the parameter types are
    /// part of the function signature, the patterns are not (they don't change
    /// the external type of the function).
    ///
    /// If this `Body` is for the body of a constant, this will just be
    /// empty.
    pub params: Vec<PatId>,
    /// The `ExprId` of the actual body expression.
    pub body_expr: ExprId,
    /// Block expressions in this body that may contain inner items.
    block_scopes: Vec<BlockId>,
    _c: Count<Self>,
}

pub type ExprPtr = AstPtr<ast::Expr>;
pub type ExprSource = InFile<ExprPtr>;

pub type PatPtr = Either<AstPtr<ast::Pat>, AstPtr<ast::SelfParam>>;
pub type PatSource = InFile<PatPtr>;

pub type LabelPtr = AstPtr<ast::Label>;
pub type LabelSource = InFile<LabelPtr>;
/// An item body together with the mapping from syntax nodes to HIR expression
/// IDs. This is needed to go from e.g. a position in a file to the HIR
/// expression containing it; but for type inference etc., we want to operate on
/// a structure that is agnostic to the actual positions of expressions in the
/// file, so that we don't recompute types whenever some whitespace is typed.
///
/// One complication here is that, due to macro expansion, a single `Body` might
/// be spread across several files. So, for each ExprId and PatId, we record
/// both the HirFileID and the position inside the file. However, we only store
/// AST -> ExprId mapping for non-macro files, as it is not clear how to handle
/// this properly for macros.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct BodySourceMap {
    expr_map: FxHashMap<ExprSource, ExprId>,
    expr_map_back: ArenaMap<ExprId, Result<ExprSource, SyntheticSyntax>>,

    pat_map: FxHashMap<PatSource, PatId>,
    pat_map_back: ArenaMap<PatId, Result<PatSource, SyntheticSyntax>>,

    label_map: FxHashMap<LabelSource, LabelId>,
    label_map_back: ArenaMap<LabelId, LabelSource>,

    /// We don't create explicit nodes for record fields (`S { record_field: 92 }`).
    /// Instead, we use id of expression (`92`) to identify the field.
    field_map: FxHashMap<InFile<AstPtr<ast::RecordExprField>>, ExprId>,
    field_map_back: FxHashMap<ExprId, InFile<AstPtr<ast::RecordExprField>>>,

    expansions: FxHashMap<InFile<AstPtr<ast::MacroCall>>, HirFileID>,

    /// Diagnostics accumulated during body lowering. These contain `AstPtr`s and so are stored in
    /// the source map (since they're just as volatile).
    diagnostics: Vec<BodyDiagnostic>,
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub struct SyntheticSyntax;

#[derive(Debug, Eq, PartialEq)]
pub enum BodyDiagnostic {
    InactiveCode {
        node: InFile<SyntaxNodePtr>,
    },
    MacroError {
        node: InFile<AstPtr<ast::MacroCall>>,
        message: String,
    },
    UnresolvedProcMacro {
        node: InFile<AstPtr<ast::MacroCall>>,
    },
    UnresolvedMacroCall {
        node: InFile<AstPtr<ast::MacroCall>>,
        path: ModPath,
    },
}

impl Body {
    pub(crate) fn body_with_source_map_query(
        db: &dyn DefDatabase,
        def: DefWithBodyId,
    ) -> (Arc<Body>, Arc<BodySourceMap>) {
        let _p = profile::span("body_with_source_map_query");
        let mut params = None;

        let (file_id, module, body) = match def {
            DefWithBodyId::FunctionId(f) => {
                let f = f.lookup(db);
                let src = f.source(db);
                params = src.value.param_list();
                (
                    src.file_id,
                    f.module(db),
                    src.value.body().map(ast::Expr::from),
                )
            }
            DefWithBodyId::ConstId(c) => {
                let c = c.lookup(db);
                let src = c.source(db);
                (src.file_id, c.module(db), src.value.body())
            }
            DefWithBodyId::StaticId(s) => {
                let s = s.lookup(db);
                let src = s.source(db);
                (src.file_id, s.module(db), src.value.body())
            }
        };
        let expander = Expander::new(db, file_id, module);
        let (mut body, source_map) = Body::new(db, expander, params, body);
        body.shrink_to_fit();
        (Arc::new(body), Arc::new(source_map))
    }

    pub(crate) fn body_query(db: &dyn DefDatabase, def: DefWithBodyId) -> Arc<Body> {
        db.body_with_source_map(def).0
    }

    /// Returns an iterator over all block expressions in this body that define inner items.
    pub fn blocks<'a>(
        &'a self,
        db: &'a dyn DefDatabase,
    ) -> impl Iterator<Item = (BlockId, Arc<DefMap>)> + '_ {
        self.block_scopes.iter().map(move |block| {
            (
                *block,
                db.block_def_map(*block).expect("block ID without DefMap"),
            )
        })
    }

    fn new(
        db: &dyn DefDatabase,
        expander: Expander,
        params: Option<ast::ParamList>,
        body: Option<ast::Expr>,
    ) -> (Body, BodySourceMap) {
        lower::lower(db, expander, params, body)
    }

    fn shrink_to_fit(&mut self) {
        let Self {
            _c: _,
            body_expr: _,
            block_scopes,
            exprs,
            labels,
            params,
            pats,
        } = self;
        block_scopes.shrink_to_fit();
        exprs.shrink_to_fit();
        labels.shrink_to_fit();
        params.shrink_to_fit();
        pats.shrink_to_fit();
    }
}

impl Index<ExprId> for Body {
    type Output = Expr;

    fn index(&self, expr: ExprId) -> &Expr {
        &self.exprs[expr]
    }
}

impl Index<PatId> for Body {
    type Output = Pat;

    fn index(&self, pat: PatId) -> &Pat {
        &self.pats[pat]
    }
}

impl Index<LabelId> for Body {
    type Output = Label;

    fn index(&self, label: LabelId) -> &Label {
        &self.labels[label]
    }
}

// FIXME: Change `node_` prefix to something more reasonable.
// Perhaps `expr_syntax` and `expr_id`?
impl BodySourceMap {
    pub fn expr_syntax(&self, expr: ExprId) -> Result<ExprSource, SyntheticSyntax> {
        self.expr_map_back[expr].clone()
    }

    pub fn node_expr(&self, node: InFile<&ast::Expr>) -> Option<ExprId> {
        let src = node.map(|it| AstPtr::new(it));
        self.expr_map.get(&src).cloned()
    }

    pub fn node_macro_file(&self, node: InFile<&ast::MacroCall>) -> Option<HirFileID> {
        let src = node.map(|it| AstPtr::new(it));
        self.expansions.get(&src).cloned()
    }

    pub fn pat_syntax(&self, pat: PatId) -> Result<PatSource, SyntheticSyntax> {
        self.pat_map_back[pat].clone()
    }

    pub fn node_pat(&self, node: InFile<&ast::Pat>) -> Option<PatId> {
        let src = node.map(|it| Either::Left(AstPtr::new(it)));
        self.pat_map.get(&src).cloned()
    }

    pub fn node_self_param(&self, node: InFile<&ast::SelfParam>) -> Option<PatId> {
        let src = node.map(|it| Either::Right(AstPtr::new(it)));
        self.pat_map.get(&src).cloned()
    }

    pub fn label_syntax(&self, label: LabelId) -> LabelSource {
        self.label_map_back[label].clone()
    }

    pub fn node_label(&self, node: InFile<&ast::Label>) -> Option<LabelId> {
        let src = node.map(|it| AstPtr::new(it));
        self.label_map.get(&src).cloned()
    }

    pub fn field_syntax(&self, expr: ExprId) -> InFile<AstPtr<ast::RecordExprField>> {
        self.field_map_back[&expr].clone()
    }
    pub fn node_field(&self, node: InFile<&ast::RecordExprField>) -> Option<ExprId> {
        let src = node.map(|it| AstPtr::new(it));
        self.field_map.get(&src).cloned()
    }

    /// Get a reference to the body source map's diagnostics.
    pub fn diagnostics(&self) -> &[BodyDiagnostic] {
        &self.diagnostics
    }
}
