use crate::line_group::produce_line_group_starts;
use crate::*;
use husky_vfs::{error::VfsResult, VirtualPath};

#[derive(Debug, PartialEq, Eq)]
pub struct TomlTokenSheet {
    tokens: Vec<TomlToken>,
    line_group_starts: Vec<usize>,
}

impl TomlTokenSheet {
    pub fn new(tokens: Vec<TomlToken>) -> Self {
        Self {
            line_group_starts: produce_line_group_starts(&tokens),
            tokens,
        }
    }

    pub fn line_groups(&self) -> impl Iterator<Item = &[TomlToken]> {
        (0..self.line_group_starts.len()).map(|line_group_index| self.line_group(line_group_index))
    }

    fn line_group(&self, line_group_index: usize) -> &[TomlToken] {
        let start = self.line_group_starts[line_group_index];
        let end = if line_group_index + 1 < self.line_group_starts.len() {
            self.line_group_starts[line_group_index + 1]
        } else {
            self.tokens.len()
        };
        &self.tokens[start..end]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TomlTokenIdx(usize);

#[salsa::tracked(return_ref)]
pub(crate) fn toml_token_sheet(
    db: &::salsa::Db,
    path: VirtualPath,
) -> VfsResult<Option<TomlTokenSheet>> {
    Ok(path
        .text(db)?
        .map(|text| TomlTokenSheet::new(db.toml_tokenize(text))))
}
