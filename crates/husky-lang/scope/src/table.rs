use crate::*;

use word::{Identifier, Keyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeTableEntry {
    ident: Option<Identifier>,
    kind: ScopeKind,
    source: ScopeSource,
}

impl ScopeTableEntry {
    pub fn parse(
        file_id: FileId,
        token_group_index: usize,
        token_group: &[token::Token],
    ) -> Result<ScopeTableEntry, ScopeDefError> {
        if token_group.len() < 2 {
            return Err(ScopeDefError {
                range: token_group[0].range.clone(),
                grammar_failed: ScopeDefGrammar::TokenGroupSizeAtLeastTwo,
            });
        }
        if token_group.len() == 2 {
            if token_group[0].kind == token::TokenKind::Keyword(Keyword::Main) {
                return Ok(ScopeTableEntry {
                    ident: None,
                    kind: ScopeKind::Routine { is_generic: false },
                    source: ScopeSource::from_file(file_id, token_group_index),
                });
            } else {
                return Err(ScopeDefError {
                    range: token_group[0].range.clone(),
                    grammar_failed: ScopeDefGrammar::TokenGroupOfSizeTwoShouldBeMain,
                });
            }
        }
        match &token_group[0].kind {
            token::TokenKind::Keyword(keyword) => {
                if let token::TokenKind::Identifier(ident) = token_group[1].kind {
                    let is_generic = token_group.len() >= 3
                        && token_group[2].kind
                            == token::TokenKind::Special(token::Special::LessOrLAngular);
                    if is_generic && !(token_group.len() >= 5) {
                        return Err(ScopeDefError {
                            range: token_group.into(),
                            grammar_failed: ScopeDefGrammar::GenericsShouldBeWellFormed,
                        });
                    } else if let Some(kind) = ScopeKind::new(*keyword, is_generic) {
                        return Ok(ScopeTableEntry {
                            ident: Some(ident),
                            kind,
                            source: ScopeSource::from_file(file_id, token_group_index),
                        });
                    }
                }
                return Err(ScopeDefError {
                    range: token_group[1].range.clone(),
                    grammar_failed: ScopeDefGrammar::NonMainSecondTokenShouldBeIdentifier,
                });
            }
            _ => Err(ScopeDefError {
                range: token_group[0].range.clone(),
                grammar_failed: ScopeDefGrammar::FirstTokenShouldBeKeyword,
            }),
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
                    Ok(entry) => Some(entry),
                    Err(new_error) => {
                        errors.push(new_error);
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
                .flatten()
            })
            .collect()
    }

    pub fn scope_source(&self, ident: Identifier) -> Option<ScopeSource> {
        self.entries
            .iter()
            .find(|entry| entry.ident == Some(ident))
            .map(|entry| entry.source)
    }

    pub fn scope_kind(&self, ident: Identifier) -> Option<ScopeKind> {
        self.entries
            .iter()
            .find(|entry| entry.ident == Some(ident))
            .map(|entry| entry.kind)
    }

    pub fn has_subscope(&self, ident: Identifier) -> bool {
        self.entries
            .iter()
            .find(|entry| entry.ident == Some(ident))
            .is_some()
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
            .filter_map(|entry| {
                entry.ident.map(|ident| Scope {
                    ident,
                    parent: ScopeParent::Scope(parent_scope_id),
                })
            })
            .collect()
    }
}
