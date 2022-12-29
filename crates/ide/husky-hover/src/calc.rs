use husky_entity_tree::EntityTreeResult;
use husky_token::Token;
use husky_token_info::TokenInfo;

use crate::*;

pub(crate) fn calc_hover_result(
    db: &dyn HoverDb,
    module_path: ModulePath,
    token_idx: TokenIdx,
) -> EntityTreeResult<Option<HoverResult>> {
    Ok(HoverResultCalculator::new(db, module_path, token_idx)?.calc_all())
}

struct HoverResultCalculator<'a> {
    db: &'a dyn HoverDb,
    module_path: ModulePath,
    token_idx: TokenIdx,
    token: &'a Token,
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
            token_info: &token_info_sheet[token_idx],
            markdown_content: String::new(),
            actions: vec![],
        })
    }

    fn calc_all(mut self) -> Option<HoverResult> {
        // todo!();
        Some(self.finish())
    }

    fn finish(self) -> HoverResult {
        HoverResult {
            hover: lsp_types::Hover {
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: self.markdown_content,
                }),
                range: Some(self.token.range.into()),
            },
            actions: self.actions,
        }
    }
}
