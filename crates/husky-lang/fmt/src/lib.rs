mod formatter;
mod query;

pub type FormattedText = folded::FoldedList<Result<String, ExprError>>;

use expr::ExprError;
pub use query::FmtQuery;
