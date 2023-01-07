use husky_entity_tree::EntityTreeResult;
use husky_expr::SymbolSheet;
use husky_text::TextRange;
use husky_token::Token;
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
        Ok(Self {
            db,
            module_path,
            token_idx,
            token: &token_sheet_data[token_idx],
            token_range: ranged_token_sheet.token_range(token_idx),
            token_info: &token_info_sheet[token_idx],
            markdown_content: String::new(),
            actions: vec![],
            hover_config_data: db.hover_config().data(db),
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
            husky_token::Keyword::Config(keyword) => "This is a config keyword.",
            husky_token::Keyword::Paradigm(keyword) => "This is a paradigm",
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
            TokenInfo::Entity(_) => format!(""),
            TokenInfo::ImplicitParameter => format!(""),
            TokenInfo::Parameter => format!(""),
            TokenInfo::LocalSymbol {
                local_symbol_idx,
                expr_sheet,
                ..
            } => {
                format!("{:#?}", expr_sheet.symbol_sheet(self.db)[*local_symbol_idx])
            }
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx,
                expr_sheet,
            } => todo!(),
            TokenInfo::Field => format!(""),
            TokenInfo::Method => format!(""),
        };
        format!(
            r#"
token_idx = {};

token = {:#?};

token_info = {:#?};

{additional_debug_content}
"#,
            self.token_idx.raw(),
            self.token,
            self.token_info,
        )
    }
}
