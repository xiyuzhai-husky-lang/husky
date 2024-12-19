mod builder;
pub mod error;
pub mod grammar;
pub mod module;
pub mod rewite_rule;

use self::{
    builder::SnlGrammarBuilder,
    error::{SnlGrammarError, SnlGrammarResult},
    grammar::SnlGrammar,
};
use eterned::db::EternerDb;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub fn grammar_from_dir(dir: impl AsRef<Path>, db: &EternerDb) -> SnlGrammarResult<SnlGrammar> {
    let dir = dir.as_ref();
    let mut builder = SnlGrammarBuilder::new(db);
    builder.scan_root(dir)?;
    Ok(builder.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect_file;

    #[test]
    fn build_snl_grammar_works() {
        fn t(relative_path: &str, db: &EternerDb) {
            let expected = expect_file!(format!("../expect-files/{}.debug.md", relative_path));
            let grammar =
                grammar_from_dir(format!("examples-data/{}", relative_path), &db).unwrap();
            expected.assert_eq(&format!(
                r#"# debug
```rust
{:#?}
```"#,
                grammar
            ));
        }

        let db = EternerDb::default();
        t("empty", &db);
        t("empty-module", &db);
        t("naive-visored", &db);
    }
}
