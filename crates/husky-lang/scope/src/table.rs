use crate::*;

use word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone)]
struct ScopeTableEntry {
    ident: Identifier,
    kind: ScopeKind,
    source: ScopeSource,
}

impl ScopeTableEntry {
    pub fn parse(
        file_id: FileId,
        token_group_index: usize,
        token_group: &[token::Token],
    ) -> Option<ScopeTableEntry> {
        if token_group.len() < 2 {
            return None;
        }
        match &token_group[0].kind {
            token::TokenKind::Keyword(keyword) => {
                if let token::TokenKind::Identifier(ident) = token_group[1].kind {
                    if let Some(kind) = ScopeKind::new(*keyword) {
                        return Some(ScopeTableEntry {
                            ident,
                            kind,
                            source: ScopeSource::from_file(file_id, token_group_index),
                        });
                    }
                }
                return None;
            }
            _ => None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ScopeTable {
    entries: Vec<ScopeTableEntry>,
}

impl ScopeTable {
    pub fn parse(file_id: FileId, token_groups: token::TokenGroupFoldedIter) -> ScopeTable {
        ScopeTable {
            entries: token_groups
                .filter_map(|(index, token_group)| {
                    ScopeTableEntry::parse(file_id, index, token_group)
                })
                .collect(),
        }
    }
    pub fn submodules(&self) -> Vec<Identifier> {
        self.entries
            .iter()
            .filter_map(|entry| {
                if entry.kind == ScopeKind::Module {
                    Some(entry.ident)
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn scope_source(&self, ident: Identifier) -> Option<ScopeSource> {
        self.entries
            .iter()
            .find(|entry| entry.ident == ident)
            .map(|entry| entry.source)
    }
    pub fn scope_kind(&self, ident: Identifier) -> Option<ScopeKind> {
        self.entries
            .iter()
            .find(|entry| entry.ident == ident)
            .map(|entry| entry.kind)
    }
}
