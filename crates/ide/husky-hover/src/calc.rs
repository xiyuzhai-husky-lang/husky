use husky_entity_tree::EntityTreeResult;
use husky_token::{Token, TokenKind};
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
                range: Some(self.token.range.into()),
            },
            actions: self.actions,
        }
    }

    fn gen_keyword_content(&self, kw: husky_token::Keyword) -> &'static str {
        r#"Bind a value to a variable.

The primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.

let thing1: i32 = 100;
let thing2 = 200 + thing1;

let mut changing_thing = true;
changing_thing = false;

let (part1, part2) = ("first", "second");

struct Example {
    a: bool,
    b: u64,
}

let Example { a, b: _ } = Example {
    a: true,
    b: 10004,
};
assert!(a);
The pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.

Variables in Rust are immutable by default, and require the mut keyword to be made mutable.

Multiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.

let shadowing_example = true;
let shadowing_example = 123.4;
let shadowing_example = shadowing_example as u32;
let mut shadowing_example = format!("cool! {shadowing_example}");
shadowing_example += " something else!"; // not shadowing
Other places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.

For more information on the let keyword, see the Rust book or the Reference"#
    }

    fn gen_content_aux(&self) -> std::borrow::Cow<'static, str> {
        match self.token.kind {
            TokenKind::Keyword(kw) => self.gen_keyword_content(kw).into(),
            _ => "".into(),
        }
    }
}
