use entity_route::*;
use file::FilePtr;
use word::{CustomIdentifier, Keyword};

use crate::error::*;
use crate::EntityRouteSalsaQueryGroup;
use lazy_format::lazy_format;
use text::TextRanged;
use token::{Token, TokenGroupIter, TokenKind};
use word::Identifier;

#[derive(PartialEq, Eq, Clone)]
pub struct Entry {
    pub ident: Option<CustomIdentifier>,
    pub kind: RawEntityKind,
    pub source: EntitySource,
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ident: {:?}, kind: {:?}}}, source: {:?}",
            self.ident, self.kind, self.source
        ))
    }
}

impl Entry {
    pub fn from_token_group(
        file_id: FilePtr,
        token_group_index: usize,
        token_group: &[Token],
    ) -> (Option<Entry>, Option<ScopeDefError>) {
        if token_group.len() < 2 {
            match token_group[0].kind {
                TokenKind::Identifier(Identifier::Custom(ident)) => {
                    return (
                        Some(Entry {
                            ident: Some(ident),
                            kind: RawEntityKind::Literal,
                            source: EntitySource::from_file(file_id, token_group_index),
                        }),
                        None,
                    )
                }
                _ => todo!(),
            }
        }
        if token_group.len() == 2 {
            if token_group[0].kind == TokenKind::Keyword(Keyword::Main.into()) {
                return (
                    Some(Entry {
                        ident: None,
                        kind: RawEntityKind::Routine,
                        source: EntitySource::from_file(file_id, token_group_index),
                    }),
                    None,
                );
            } else {
                return (None, None);
            }
        }
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                if let TokenKind::Identifier(ident) = token_group[1].kind {
                    if let Some(kind) = RawEntityKind::new(keyword, &token_group[2]) {
                        return match ident {
                            Identifier::Builtin(_) => (
                                None,
                                Some(ScopeDefError {
                                    range: token_group[1].text_range(),
                                    rule_broken: ScopeDefRule::BuiltinIdentifierAreReserved,
                                }),
                            ),
                            Identifier::Custom(user_defined_ident) => (
                                Some(Entry {
                                    ident: Some(user_defined_ident),
                                    kind,
                                    source: EntitySource::from_file(file_id, token_group_index),
                                }),
                                None,
                            ),
                            Identifier::Contextual(_) => (
                                None,
                                Some(ScopeDefError {
                                    range: token_group[1].text_range(),
                                    rule_broken: ScopeDefRule::ContextualIdentifierAreReserved,
                                }),
                            ),
                        };
                    }
                }
                (
                    None,
                    Some(ScopeDefError {
                        range: token_group[1].text_range(),
                        rule_broken: ScopeDefRule::NonMainSecondTokenShouldBeIdentifier,
                    }),
                )
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

impl std::fmt::Display for SubscopeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.entries
            .iter()
            .map(|entry| f.write_fmt(format_args!("{:?},", entry)))
            .collect::<std::fmt::Result>()?;
        f.write_str("[")?;
        Ok(())
    }
}

impl SubscopeTable {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(file_id: FilePtr, token_groups: TokenGroupIter) -> Self {
        let mut errors = Vec::new();
        let entries = token_groups
            .filter_map(|item| {
                let (entry, error) = Entry::from_token_group(file_id, item.idx, item.value);
                error.map(|error| errors.push(error));
                entry
            })
            .collect();
        Self { entries, errors }
    }
}

impl SubscopeTable {
    pub fn submodule_idents(&self) -> Vec<CustomIdentifier> {
        self.entries
            .iter()
            .filter_map(|entry| {
                if entry.kind == RawEntityKind::Module {
                    Some(entry.ident)
                } else {
                    None
                }
                .flatten()
            })
            .collect()
    }

    pub fn scope_source(&self, ident: CustomIdentifier) -> ScopeResult<EntitySource> {
        not_none!(
            self.entries
                .iter()
                .find(|entry| entry.ident == Some(ident))
                .map(|entry| entry.source),
            format!("No scope with ident: \"{}\" among {}", ident, self)
        )
    }

    pub fn raw_entity_kind(&self, ident: CustomIdentifier) -> Option<RawEntityKind> {
        self.entries
            .iter()
            .find(|entry| entry.ident == Some(ident))
            .map(|entry| entry.kind)
    }

    pub fn has_subscope(
        &self,
        ident: CustomIdentifier,
        generic_arguments: &[GenericArgument],
    ) -> bool {
        if generic_arguments.len() > 0 {
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
    pub fn subscopes(&self, parent_scope_id: EntityRoutePtr) -> Vec<Route> {
        self.entries
            .iter()
            .filter_map(|entry| {
                entry
                    .ident
                    .map(|ident| Route::child_scope(parent_scope_id, ident, Vec::new()))
            })
            .collect()
    }
}

impl SubscopeTable {
    pub(crate) fn builtin(this: &dyn EntityRouteSalsaQueryGroup, data: &BuiltinEntityData) -> Self {
        let entries = data
            .subscopes
            .iter()
            .map(|(s, data)| Entry {
                ident: Some(this.intern_word(s).custom().unwrap()),
                kind: data.signature.raw_entity_kind(),
                source: (*data).into(),
            })
            .collect();
        Self {
            entries,
            errors: vec![],
        }
    }
}
