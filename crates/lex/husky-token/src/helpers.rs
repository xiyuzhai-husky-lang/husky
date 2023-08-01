mod context;
mod ident;
mod keyword;
mod label;
mod path_name;
mod punctuation;
mod symbol_modifier;

pub use self::context::*;
pub use self::ident::*;
pub use self::keyword::*;
pub use self::label::*;
pub use self::path_name::*;
pub use self::punctuation::*;
pub use self::symbol_modifier::*;

use crate::*;
use parsec::*;

#[cfg(test)]
fn quick_parse<T, Error>(db: &DB, input: &str) -> Result<Option<T>, Error>
where
    T: for<'a> TryParseOptionFromStream<TokenStream<'a>, Error = Error>,
{
    let token_sheet = db.snippet_token_sheet_data(Snippet::new(db, input.to_owned()));
    let mut stream = token_sheet
        .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    stream.try_parse_option()
}
