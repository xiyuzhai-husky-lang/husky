use husky_entity_tree::EntityTreeResult;
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
}

impl<'a> HoverResultCalculator<'a> {
    fn new(
        db: &'a dyn HoverDb,
        module_path: ModulePath,
        token_idx: TokenIdx,
    ) -> EntityTreeResult<Self> {
        let token_sheet = db.token_sheet(module_path)?;
        let token_info_sheet = db.token_info_sheet(module_path)?;
        Ok(Self {
            db,
            module_path,
            token_idx,
            token: &token_sheet[token_idx],
            token_range: token_sheet.token_range(token_idx),
            token_info: &token_info_sheet[token_idx],
            markdown_content: String::new(),
            actions: vec![],
        })
    }

    fn gen_content(mut self) -> Option<HoverResult> {
        self.markdown_content += &self.gen_content_aux();
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

    fn gen_content_aux(&self) -> std::borrow::Cow<'static, str> {
        match self.token {
            Token::Keyword(kw) => self.gen_keyword_content(*kw).into(),
            _ => "".into(),
        }
    }
}
