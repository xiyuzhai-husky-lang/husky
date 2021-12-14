use crate::*;

use word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeTableEntry {
    ident: Identifier,
    kind: ScopeKind,
    source: ScopeSource,
}

impl ScopeTableEntry {
    pub fn parse(
        file_id: FileId,
        token_group_index: usize,
        token_group: &[token::Token],
    ) -> Result<ScopeTableEntry, Vec<ScopeDefError>> {
        if token_group.len() < 2 {
            return Err(vec![ScopeDefError {
                range: token_group[0].range.clone(),
                grammar_failed: ScopeDefGrammar::TokenGroupSizeAtLeastTwo,
            }]);
        }
        match &token_group[0].kind {
            token::TokenKind::Keyword(keyword) => {
                if let token::TokenKind::Identifier(ident) = token_group[1].kind {
                    if let Some(kind) = ScopeKind::new(*keyword) {
                        return Ok(ScopeTableEntry {
                            ident,
                            kind,
                            source: ScopeSource::from_file(file_id, token_group_index),
                        });
                    }
                }
                return Err(vec![ScopeDefError {
                    range: token_group[1].range.clone(),
                    grammar_failed: ScopeDefGrammar::SecondTokenShouldBeIdentifier,
                }]);
            }
            _ => Err(vec![ScopeDefError {
                range: token_group[0].range.clone(),
                grammar_failed: ScopeDefGrammar::FirstTokenShouldBeKeyword,
            }]),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubscopeTable {
    entries: Vec<ScopeTableEntry>,
    errors: Vec<ScopeDefError>,
}

impl SubscopeTable {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(file_id: FileId, token_groups: token::TokenGroupFoldedIter) -> Self {
        let mut errors = vec![];
        let entries = token_groups
            .filter_map(|(index, token_group)| {
                match ScopeTableEntry::parse(file_id, index, token_group) {
                    Ok(_) => todo!(),
                    Err(new_errors) => {
                        errors.extend(new_errors);
                        None
                    }
                }
            })
            .collect();
        Self { entries, errors }
    }
}

impl SubscopeTable {
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

impl SubscopeTable {
    pub fn entry_iter(&self) -> core::slice::Iter<ScopeTableEntry> {
        self.entries.iter()
    }
    pub fn error_iter(&self) -> core::slice::Iter<ScopeDefError> {
        self.errors.iter()
    }
    pub fn subscopes(&self, parent_scope_id: ScopeId) -> Vec<Scope> {
        self.entries
            .iter()
            .map(|entry| Scope {
                ident: entry.ident,
                parent: ScopeParent::Scope(parent_scope_id),
            })
            .collect()
    }
}
