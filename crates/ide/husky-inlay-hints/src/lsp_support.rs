use crate::*;
use crate_decl::crate_decl_inlay_hints;
use husky_entity_path::{path::ItemPath, region::RegionPath};
use husky_entity_tree::helpers::{paths::module_item_paths, tokra_region::HasRegionalTokenIdxBase};
use husky_regional_token::RegionalTokenIdxBase;
use husky_text::Text;
use husky_token::sheet::RangedTokenSheet;
use husky_vfs::path::{crate_path::CratePath, module_path::ModulePathData};
use item_decl::item_decl_inlay_hints;
use item_defn::item_defn_inlay_hints;

#[sealed::sealed]
pub trait HasLspInlayHints: Is<ModulePath> + Copy {
    fn lsp_inlay_hints(
        self,
        db: &::salsa::Db,
        range: Option<TextRange>,
    ) -> InlayHintResult<Option<Vec<lsp_types::InlayHint>>>;
}

#[sealed::sealed]
impl HasLspInlayHints for ModulePath {
    fn lsp_inlay_hints(
        self,
        db: &salsa::Db,
        range: Option<TextRange>,
    ) -> InlayHintResult<Option<Vec<lsp_types::InlayHint>>> {
        let mut builder = LspInlayHintBuilder::new(self, db);
        match self.data(db) {
            ModulePathData::Root(crate_path) => builder.add_crate_decl(crate_path),
            ModulePathData::Child { parent, ident } => (),
            ModulePathData::Chunk { chunk } => (),
        }
        for &item_path in module_item_paths(db, self) {
            builder.add_item_decl_and_defn(item_path)
        }
        Ok(Some(builder.finish()))
    }
}

struct LspInlayHintBuilder<'db> {
    db: &'db ::salsa::Db,
    base: Option<RegionalTokenIdxBase>,
    text: Text<'db>,
    token_range_sheet_data: &'db RangedTokenSheet,
    lsp_inlay_hints: Vec<lsp_types::InlayHint>,
}

impl<'db> LspInlayHintBuilder<'db> {
    fn new(module_path: ModulePath, db: &'db ::salsa::Db) -> Self {
        Self {
            db,
            base: todo!(),
            text: todo!(),
            token_range_sheet_data: todo!(),
            lsp_inlay_hints: todo!(),
        }
    }
}

impl<'db> LspInlayHintBuilder<'db> {
    #[track_caller]
    fn base(&self) -> RegionalTokenIdxBase {
        self.base.unwrap()
    }
}

/// # actions
impl<'db> LspInlayHintBuilder<'db> {
    fn add_crate_decl(&mut self, crate_path: CratePath) {
        let db = self.db;
        self.base = RegionPath::CrateDecl(crate_path).regional_token_idx_base(db);
        self.add(crate_decl_inlay_hints(db, crate_path));
    }

    fn add_item_decl_and_defn(&mut self, item_path: ItemPath) {
        let db = self.db;
        debug_assert!(self.base.is_none());
        self.base = RegionPath::ItemDecl(item_path).regional_token_idx_base(db);
        self.add(item_decl_inlay_hints(db, *item_path));
        self.base = RegionPath::ItemDefn(item_path).regional_token_idx_base(db);
        self.add(item_defn_inlay_hints(db, *item_path));
    }

    fn add(&mut self, inlay_hints: &'db [InlayHint]) {
        todo!()
    }

    fn finish(self) -> Vec<lsp_types::InlayHint> {
        self.lsp_inlay_hints
    }
}

impl InlayHint {
    fn lsp(&self, builder: &LspInlayHintBuilder) -> lsp_types::InlayHint {
        lsp_types::InlayHint {
            position: builder
                .token_range_sheet_data
                .token_text_range(self.position.token_idx(builder.base()))
                .start
                .into(),
            label: self.label.lsp(),
            kind: self.kind.into(),
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: None,
            data: None,
        }
    }
}

impl Into<Option<lsp_types::InlayHintKind>> for InlayHintKind {
    fn into(self) -> Option<lsp_types::InlayHintKind> {
        match self {
            InlayHintKind::TypeHint => Some(lsp_types::InlayHintKind::TYPE),
            InlayHintKind::ParameterHint => Some(lsp_types::InlayHintKind::PARAMETER),
            InlayHintKind::ChainingHint => None,
        }
    }
}

impl InlayHintLabel {
    fn lsp(&self) -> lsp_types::InlayHintLabel {
        match self {
            InlayHintLabel::String(label) => lsp_types::InlayHintLabel::String(label.clone()),
            InlayHintLabel::LabelParts(parts) => lsp_types::InlayHintLabel::LabelParts(todo!()),
        }
    }
}
