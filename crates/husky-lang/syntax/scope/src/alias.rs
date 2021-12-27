use token::TokenKind;
use word::Keyword;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeAliasTable {
    entries: Vec<Entry>,
    errors: Vec<ScopeAliasDefError>,
}

impl ScopeAliasTable {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(file_id: FileId, token_groups: token::TokenGroupIter) -> Self {
        let mut errors = Vec::new();
        let entries = token_groups
            .filter_map(|(index, token_group, _)| {
                let (entry, error) = Entry::parse(file_id, index, token_group);
                error.map(|error| errors.push(error));
                entry
            })
            .collect();
        Self { entries, errors }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    ident: Identifier,
    scope_id: ScopeId,
}

impl Entry {
    pub fn parse(
        _file_id: FileId,
        _token_group_index: usize,
        token_group: &[token::Token],
    ) -> (Option<Entry>, Option<ScopeAliasDefError>) {
        if token_group[0].kind == TokenKind::Keyword(Keyword::Use) {
            todo!()
        } else {
            (None, None)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeAliasDefError {}
