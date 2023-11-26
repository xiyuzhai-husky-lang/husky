use husky_entity_syn_tree::EntitySynTreeResult;
use husky_text_protocol::range::TextRange;

use husky_token::{TokenDb, TokenGroupIdx};
use husky_token_data::{Keyword, TokenData};
use husky_token_info::{TokenInfo, TokenInfoData, TokenInfoDb};

use crate::*;

pub(crate) fn calc_hover_result(
    db: &::salsa::Db,
    module_path: ModulePath,
    token_idx: TokenIdx,
) -> Option<HoverResult> {
    HoverResultCalculator::new(db, module_path, token_idx)
        .ok()?
        .gen_content()
}

struct HoverResultCalculator<'a> {
    db: &'a ::salsa::Db,
    module_path: ModulePath,
    token_idx: TokenIdx,
    token: &'a TokenData,
    token_range: TextRange,
    token_info: Option<&'a TokenInfo>,
    markdown_content: String,
    actions: Vec<CommandLinkGroup>,
    hover_config_data: &'a HoverConfigData,
    token_group_idx: TokenGroupIdx,
}

impl<'a> HoverResultCalculator<'a> {
    fn new(
        db: &'a ::salsa::Db,
        module_path: ModulePath,
        token_idx: TokenIdx,
    ) -> EntitySynTreeResult<Self> {
        let ranged_token_sheet = db.ranged_token_sheet(module_path);
        let token_sheet_data = db.token_sheet_data(module_path);
        let token_info_sheet = db.token_info_sheet(module_path)?;
        let token_group_idx = token_sheet_data.token_group_idx(token_idx);
        Ok(Self {
            db,
            module_path,
            token_idx,
            token: &token_sheet_data[token_idx],
            token_range: ranged_token_sheet.token_text_range(token_idx),
            token_info: token_info_sheet[token_idx].as_ref(),
            markdown_content: String::new(),
            actions: vec![],
            hover_config_data: db.hover_config().data(db),
            token_group_idx,
        })
    }

    fn gen_content(mut self) -> Option<HoverResult> {
        self.markdown_content += &self.content();
        if self.hover_config_data.debug {
            self.markdown_content += &self.debug_content()
        }
        Some(self.finish())
    }

    fn finish(self) -> HoverResult {
        HoverResult {
            hover: lsp_types::Hover {
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: self.markdown_content,
                }),
                range: Some(self.token_range.into()),
            },
            actions: self.actions,
        }
    }

    fn gen_keyword_content(&self, kw: Keyword) -> &'static str {
        match kw {
            Keyword::Fugitive(_keyword) => "This is a paradigm",
            _ => "Other",
        }
    }

    fn content(&self) -> std::borrow::Cow<'static, str> {
        match self.token {
            TokenData::Keyword(kw) => self.gen_keyword_content(*kw).into(),
            _ => "".into(),
        }
    }

    fn debug_content(&self) -> String {
        let additional_debug_content: String = match self.token_info {
            Some(info) => match info.data() {
                TokenInfoData::Entity(_) => format!(""),
                TokenInfoData::EntityNode(_, _) => format!(""),
                TokenInfoData::CurrentSynSymbol {
                    current_syn_symbol_idx,
                    syn_expr_region,
                    ..
                } => {
                    format!(
                        "{:#?}",
                        syn_expr_region.data(self.db).symbol_region()[*current_syn_symbol_idx]
                            .debug(self.db)
                    )
                }
                TokenInfoData::InheritedSynSymbol {
                    inherited_syn_symbol_idx,
                    syn_expr_region,
                    ..
                } => {
                    format!(
                        "{:#?}",
                        syn_expr_region.data(self.db).symbol_region()[*inherited_syn_symbol_idx]
                            .debug(self.db)
                    )
                }
                TokenInfoData::Field => format!(""),
                TokenInfoData::Method => format!(""),
                TokenInfoData::BoxColon => format!("box colon"),
                TokenInfoData::VecFunctorBoxPrefix => format!("vec functor box prefix"),
                TokenInfoData::ArrayFunctorBoxPrefix => format!("array functor box prefix"),
                TokenInfoData::UseExpr { .. } => format!("use"),
                TokenInfoData::UseExprStar => format!("use expr star"),
                TokenInfoData::SelfType => format!("self type"),
                TokenInfoData::SelfValue => format!("self value"),
                TokenInfoData::HtmlFunctionIdent => format!("html function ident"),
                TokenInfoData::HtmlPropertyIdent => format!("html property ident"),
                TokenInfoData::UnitLeftParenthesis => format!("unit `(`"),
                TokenInfoData::UnitRightParenthesis => format!("unit `)`"),
                TokenInfoData::Todo => format!("todo"),
                TokenInfoData::Unreachable => format!("unreachable"),
                TokenInfoData::SemaPrefixTypeOpr => format!("SemaPrefixTypeOpr"),
                TokenInfoData::CallPar => format!("call par"),
            },
            None => format!(""),
        };
        format!(
            r#"
token_idx = {};

token_line_group_idx = {}

token = {:#?};

token_info = {:#?};

{additional_debug_content}
"#,
            self.token_idx.index(),
            self.token_group_idx,
            self.token.debug(self.db),
            self.token_info.debug(self.db),
        )
    }
}
