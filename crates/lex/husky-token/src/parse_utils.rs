mod context;
mod identifier;
mod keyword;
mod punctuation;

pub use context::*;
pub use identifier::*;
pub use keyword::*;
pub use punctuation::*;

use crate::*;
use parsec::*;

#[cfg(test)]
fn quick_parse<T>(db: &DB, input: &str) -> TokenResult<Option<T>>
where
    T: for<'a> ParseFrom<TokenStream<'a>>,
{
    let token_sheet = db.snippet_token_sheet_data(Snippet::new(db, input.to_owned()));
    let mut stream = token_sheet
        .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    stream.parse()
}
