use crate::*;

use crate::error::*;

use token::{Special, TokenKind};
use word::{Identifier, Keyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    ident: Option<Identifier>,
    kind: ScopeKind,
    source: ScopeSource,
}

impl Entry {
    pub fn parse(
        file_id: FileId,
        token_group_index: usize,
        token_group: &[token::Token],
    ) -> (Option<Entry>, Option<ScopeDefError>) {
        if token_group.len() < 2 {
            return (
                None,
                Some(ScopeDefError {
                    range: token_group[0].range.clone(),
                    grammar_failed: ScopeDefGrammar::TokenGroupSizeAtLeastTwo,
                }),
            );
        }
        if token_group.len() == 2 {
            if token_group[0].kind == TokenKind::Keyword(Keyword::Main) {
                let error = if token_group[1].kind != TokenKind::Special(Special::Colon) {
                    todo!()
                } else {
                    None
                };
                return (
                    Some(Entry {
                        ident: None,
                        kind: ScopeKind::Routine { is_generic: false },
                        source: ScopeSource::from_file(file_id, token_group_index),
                    }),
                    error,
                );
            } else {
                return (
                    None,
                    Some(ScopeDefError {
                        range: token_group[0].range.clone(),
                        grammar_failed: ScopeDefGrammar::TokenGroupOfSizeTwoShouldBeMain,
                    }),
                );
            }
        }
        match &token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                if let TokenKind::Identifier(ident) = token_group[1].kind {
                    let is_generic = token_group.len() >= 3
                        && token_group[2].kind == TokenKind::Special(Special::LessOrLAngular);
                    if is_generic && !(token_group.len() >= 5) {
                        return (
                            None,
                            Some(ScopeDefError {
                                range: token_group.into(),
                                grammar_failed: ScopeDefGrammar::GenericsShouldBeWellFormed,
                            }),
                        );
                    } else if let Some(kind) = ScopeKind::new(*keyword, is_generic) {
                        return (
                            Some(Entry {
                                ident: Some(ident),
                                kind,
                                source: ScopeSource::from_file(file_id, token_group_index),
                            }),
                            None,
                        );
                    }
                }
                return (
                    None,
                    Some(ScopeDefError {
                        range: token_group[1].range.clone(),
                        grammar_failed: ScopeDefGrammar::NonMainSecondTokenShouldBeIdentifier,
                    }),
                );
            }
            _ => (
                None,
                Some(ScopeDefError {
                    range: token_group[0].range.clone(),
                    grammar_failed: ScopeDefGrammar::FirstTokenShouldBeKeyword,
                }),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubscopeTable {
    entries: Vec<Entry>,
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
        let mut errors = Vec::new();
        let entries = token_groups
            .filter_map(|(index, token_group)| {
                let (entry, error) = Entry::parse(file_id, index, token_group);
                error.map(|error| errors.push(error));
                entry
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

    pub fn scope_source(&self, ident: Identifier) -> ScopeResult<ScopeSource> {
        self.entries
            .iter()
            .find(|entry| entry.ident == Some(ident))
            .map(|entry| entry.source)
            .ok_or(ScopeError::NoSuchScope)
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
    pub fn entry_iter(&self) -> core::slice::Iter<Entry> {
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
