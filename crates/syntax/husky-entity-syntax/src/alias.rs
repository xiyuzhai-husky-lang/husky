use husky_file::FileItd;
use husky_term::Ty;
use husky_token::TokenKind;
use husky_word::{Identifier, Keyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityRouteAliasTable {
    entries: Vec<EntityRouteAliasEntry>,
    errors: Vec<ScopeAliasDefError>,
}

impl EntityRouteAliasTable {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(file_id: FileItd, token_groups: husky_token::TokenGroupIter) -> Self {
        let mut errors = Vec::new();
        let entries = token_groups
            .filter_map(|item| {
                let (entry, error) = EntityRouteAliasEntry::parse(file_id, item.idx, item.value);
                error.map(|error| errors.push(error));
                entry
            })
            .collect();
        Self { entries, errors }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityRouteAliasEntry {
    ident: Identifier,
    scope_id: Ty,
}

impl EntityRouteAliasEntry {
    pub fn parse(
        _file_id: FileItd,
        _token_group_index: usize,
        token_group: &[husky_token::Token],
    ) -> (Option<EntityRouteAliasEntry>, Option<ScopeAliasDefError>) {
        if token_group[0].kind == TokenKind::Keyword(Keyword::Use) {
            todo!()
        } else {
            (None, None)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeAliasDefError {}
