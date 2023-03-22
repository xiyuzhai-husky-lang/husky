use husky_entity_tree::EntityTreeResult;
use husky_text::TextRange;

use husky_token::{RangedTokenSheet, Token, TokenGroupIdx, TokenSheetData};
use husky_token_info::TokenInfo;

use crate::*;

pub(crate) fn calc_hover_result(
    db: &dyn HoverDb,
    module_path: ModulePath,
    token_idx: TokenIdx,
) -> EntityTreeResult<Option<HoverResult>> {
    Ok(HoverResultCalculator::new(db, module_path, token_idx)?.gen_content())
}

struct HoverResultCalculator<'a> {
    db: &'a dyn HoverDb,
    module_path: ModulePath,
    token_idx: TokenIdx,
    token: &'a Token,
    token_range: TextRange,
    token_info: &'a TokenInfo,
    markdown_content: String,
    actions: Vec<CommandLinkGroup>,
    hover_config_data: &'a HoverConfigData,
    token_group_idx: TokenGroupIdx,
}

impl<'a> HoverResultCalculator<'a> {
    fn new(
        db: &'a dyn HoverDb,
        module_path: ModulePath,
        token_idx: TokenIdx,
    ) -> EntityTreeResult<Self> {
        let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
        let token_sheet_data = db.token_sheet_data(module_path)?;
        let token_info_sheet = db.token_info_sheet(module_path)?;
        let token_group_idx = token_sheet_data.token_group_idx(token_idx);
        Ok(Self {
            db,
            module_path,
            token_idx,
            token: &token_sheet_data[token_idx],
            token_range: ranged_token_sheet.token_text_range(token_idx),
            token_info: &token_info_sheet[token_idx],
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

    fn gen_keyword_content(&self, kw: husky_token::Keyword) -> &'static str {
        match kw {
            husky_token::Keyword::Config(_keyword) => "This is a config keyword.",
            husky_token::Keyword::Form(_keyword) => "This is a paradigm",
            _ => "Other",
        }
    }

    fn content(&self) -> std::borrow::Cow<'static, str> {
        match self.token {
            Token::Keyword(kw) => self.gen_keyword_content(*kw).into(),
            _ => "".into(),
        }
    }

    fn debug_content(&self) -> String {
        let additional_debug_content: String = match self.token_info {
            TokenInfo::None => format!(""),
            TokenInfo::Entity(_, _) => format!(""),
            TokenInfo::CurrentSymbol {
                current_symbol_idx,
                expr_region,
                ..
            } => {
                format!(
                    "{:#?}",
                    expr_region.data(self.db).symbol_region()[*current_symbol_idx].debug(self.db)
                )
            }
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx,
                expr_region,
                ..
            } => {
                format!(
                    "{:#?}",
                    expr_region.data(self.db).symbol_region()[*inherited_symbol_idx].debug(self.db)
                )
            }
            TokenInfo::Field => format!(""),
            TokenInfo::Method => format!(""),
            TokenInfo::BoxColon => format!("box colon"),
            TokenInfo::BoxPrefix => format!("box prefix"),
            TokenInfo::UseExpr { .. } => format!("use"),
            TokenInfo::UseExprStar => format!("use expr star"),
            TokenInfo::SelfType => format!("self type"),
            TokenInfo::SelfValue => format!("self value"),
        };
        format!(
            r#"
token_idx = {};

token_line_group_idx = {}

token = {:#?};

token_info = {:#?};

{additional_debug_content}
"#,
            self.token_idx.raw(),
            self.token_group_idx,
            self.token.debug(self.db),
            self.token_info.debug(self.db),
        )
    }
}
