use crate::*;

use crate::error::*;

use text::get_slice_text_range;
use text::HasTextRange;
use token::{Special, Token, TokenGroupIter, TokenKind};
use word::{Identifier, Keyword};

#[derive(PartialEq, Eq, Clone)]
pub struct Entry {
    ident: Option<UserDefinedIdentifier>,
    kind: ScopeKind,
    source: ScopeSource,
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ident: {:?}, kind: {:?}}}, source: {:?}",
            self.ident, self.kind, self.source
        ))
    }
}

impl Entry {
    pub fn parse(
        file_id: FileId,
        token_group_index: usize,
        token_group: &[Token],
    ) -> (Option<Entry>, Option<ScopeDefError>) {
        if token_group.len() < 2 {
            return (
                None,
                Some(ScopeDefError {
                    range: token_group[0].text_range(),
                    rule_broken: ScopeDefRule::TokenGroupSizeAtLeastTwo,
                }),
            );
        }
        if token_group.len() == 2 && token_group[0].kind == TokenKind::Keyword(Keyword::Main) {
            return (
                Some(Entry {
                    ident: None,
                    kind: ScopeKind::Routine,
                    source: ScopeSource::from_file(file_id, token_group_index),
                }),
                None,
            );
        }
        match &token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                if let TokenKind::Identifier(ident) = token_group[1].kind {
                    if let Some(kind) = ScopeKind::new(*keyword) {
                        match ident {
                            Identifier::Builtin(_) => {
                                return (
                                    None,
                                    Some(ScopeDefError {
                                        range: token_group[1].text_range(),
                                        rule_broken: ScopeDefRule::BuiltinIdentifierAreReserved,
                                    }),
                                )
                            }
                            Identifier::UserDefined(user_defined_ident) => {
                                return (
                                    Some(Entry {
                                        ident: Some(user_defined_ident),
                                        kind,
                                        source: ScopeSource::from_file(file_id, token_group_index),
                                    }),
                                    None,
                                )
                            }
                        }
                    }
                }
                return (
                    None,
                    Some(ScopeDefError {
                        range: token_group[1].text_range(),
                        rule_broken: ScopeDefRule::NonMainSecondTokenShouldBeIdentifier,
                    }),
                );
            }
            _ => (
                None,
                Some(ScopeDefError {
                    range: token_group[0].text_range(),
                    rule_broken: ScopeDefRule::FirstTokenShouldBeKeyword,
                }),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubscopeTable {
    pub entries: Vec<Entry>,
    pub errors: Vec<ScopeDefError>,
}

impl SubscopeTable {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(file_id: FileId, token_groups: TokenGroupIter) -> Self {
        let mut errors = Vec::new();
        let entries = token_groups
            .filter_map(|(index, token_range)| {
                let (entry, error) = Entry::parse(file_id, index, token_range);
                error.map(|error| errors.push(error));
                entry
            })
            .collect();
        Self { entries, errors }
    }
}

impl SubscopeTable {
    pub fn submodule_idents(&self) -> Vec<UserDefinedIdentifier> {
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

    pub fn scope_source(&self, ident: UserDefinedIdentifier) -> ScopeResult<ScopeSource> {
        self.entries
            .iter()
            .find(|entry| entry.ident == Some(ident))
            .map(|entry| entry.source)
            .ok_or(ScopeError::NoSuchScope)
    }

    pub fn scope_kind(&self, ident: UserDefinedIdentifier) -> Option<ScopeKind> {
        self.entries
            .iter()
            .find(|entry| entry.ident == Some(ident))
            .map(|entry| entry.kind)
    }

    pub fn has_subscope(
        &self,
        ident: UserDefinedIdentifier,
        generic_arguments: &Option<Vec<ScopeId>>,
    ) -> bool {
        if generic_arguments.is_some() {
            todo!()
        }
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
                entry
                    .ident
                    .map(|ident| Scope::child_scope(parent_scope_id, ident, None))
            })
            .collect()
    }
}
