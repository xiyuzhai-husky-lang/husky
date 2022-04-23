use dev_utils::dev_src;
use entity_route::*;
use file::FilePtr;
use static_defn::*;
use word::{CustomIdentifier, Keyword};

use crate::error::*;
use crate::EntityRouteSalsaQueryGroup;
use text::TextRanged;
use token::{Token, TokenGroupIter, TokenKind};
use word::Identifier;

#[derive(PartialEq, Eq, Clone)]
pub struct Entry {
    pub ident: Option<CustomIdentifier>,
    pub kind: EntityKind,
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
        file: FilePtr,
        token_group_index: usize,
        token_group: &[Token],
    ) -> (Option<Entry>, Option<EntityDefnError>) {
        if token_group.len() < 2 {
            match token_group[0].kind {
                TokenKind::Identifier(Identifier::Custom(ident)) => {
                    return (
                        Some(Entry {
                            ident: Some(ident),
                            kind: EntityKind::Literal,
                            source: EntitySource::from_file(file, token_group_index),
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
                        kind: EntityKind::Routine,
                        source: EntitySource::from_file(file, token_group_index),
                    }),
                    None,
                );
            } else if token_group[0].kind == TokenKind::Keyword(Keyword::Mod.into()) {
                return match token_group[1].kind {
                    TokenKind::Keyword(_) => todo!(),
                    TokenKind::Identifier(ident) => (
                        Some(Entry {
                            ident: Some(ident.opt_custom().expect("todo")),
                            kind: EntityKind::Module,
                            source: EntitySource::WithinModule {
                                file,
                                token_group_index: token_group_index,
                            },
                        }),
                        None,
                    ),
                    TokenKind::Special(_) => todo!(),
                    TokenKind::I32Literal(_) => todo!(),
                    TokenKind::F32Literal(_) => todo!(),
                };
            } else {
                return (None, None);
            }
        }
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                if let TokenKind::Identifier(ident) = token_group[1].kind {
                    if let Some(kind) = EntityKind::new(keyword, &token_group[2]) {
                        return match ident {
                            Identifier::Builtin(_) => (
                                None,
                                Some(EntityDefnError {
                                    range: token_group[1].text_range(),
                                    rule_broken: ScopeDefRule::BuiltinIdentifierAreReserved,
                                    dev_src: dev_src!(),
                                }),
                            ),
                            Identifier::Custom(user_defined_ident) => (
                                Some(Entry {
                                    ident: Some(user_defined_ident),
                                    kind,
                                    source: EntitySource::from_file(file, token_group_index),
                                }),
                                None,
                            ),
                            Identifier::Contextual(_) => (
                                None,
                                Some(EntityDefnError {
                                    range: token_group[1].text_range(),
                                    rule_broken: ScopeDefRule::ContextualIdentifierAreReserved,
                                    dev_src: dev_src!(),
                                }),
                            ),
                        };
                    }
                }
                (
                    None,
                    Some(EntityDefnError {
                        range: token_group[1].text_range(),
                        rule_broken: ScopeDefRule::NonMainSecondTokenShouldBeIdentifier,
                        dev_src: dev_src!(),
                    }),
                )
            }
            _ => (
                None,
                Some(EntityDefnError {
                    range: token_group[0].text_range(),
                    rule_broken: ScopeDefRule::FirstTokenShouldBeKeyword,
                    dev_src: dev_src!(),
                }),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChildRouteTable {
    pub entries: Vec<Entry>,
    pub errors: Vec<EntityDefnError>,
}

impl std::fmt::Display for ChildRouteTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.entries
            .iter()
            .map(|entry| f.write_fmt(format_args!("{:?},", entry)))
            .collect::<std::fmt::Result>()?;
        f.write_str("]")?;
        Ok(())
    }
}

impl ChildRouteTable {
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

impl ChildRouteTable {
    pub fn submodule_idents(&self) -> Vec<CustomIdentifier> {
        self.entries
            .iter()
            .filter_map(|entry| {
                if entry.kind == EntityKind::Module {
                    Some(entry.ident)
                } else {
                    None
                }
                .flatten()
            })
            .collect()
    }

    pub fn entity_source(&self, ident: CustomIdentifier) -> EntityRouteResult<EntitySource> {
        not_none!(
            self.entries
                .iter()
                .find(|entry| entry.ident == Some(ident))
                .map(|entry| entry.source),
            format!("No entity route with ident: \"{}\" among {}", ident, self)
        )
    }

    pub fn raw_entity_kind(&self, ident: CustomIdentifier) -> Option<EntityKind> {
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

impl ChildRouteTable {
    pub fn entry_iter(&self) -> core::slice::Iter<Entry> {
        self.entries.iter()
    }
    pub fn error_iter(&self) -> core::slice::Iter<EntityDefnError> {
        self.errors.iter()
    }
    pub fn child_routes(&self, parent_scope_id: EntityRoutePtr) -> Vec<EntityRoute> {
        self.entries
            .iter()
            .filter_map(|entry| {
                entry
                    .ident
                    .map(|ident| EntityRoute::child_route(parent_scope_id, ident, Vec::new()))
            })
            .collect()
    }
}

impl ChildRouteTable {
    pub(crate) fn from_static(
        db: &dyn EntityRouteSalsaQueryGroup,
        data: &EntityStaticDefn,
    ) -> Self {
        let mut entries: Vec<Entry> = data
            .subscopes
            .iter()
            .map(|(s, data)| Entry {
                ident: Some(db.intern_word(s).opt_custom().unwrap()),
                kind: data.variant.raw_entity_kind(),
                source: (*data).into(),
            })
            .collect();
        match data.variant {
            EntityStaticDefnVariant::Routine { .. } | EntityStaticDefnVariant::Module => (),
            EntityStaticDefnVariant::Type {
                type_members,
                variants,
                ..
            } => {
                for type_member in type_members {
                    entries.push(Entry {
                        ident: Some(db.intern_word(type_member.name).custom()),
                        kind: EntityKind::TypeMember,
                        source: EntitySource::StaticTypeMember,
                    })
                }
                for variant in variants {
                    todo!()
                }
            }
            EntityStaticDefnVariant::Trait { .. } => todo!(),
            EntityStaticDefnVariant::Method { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TypeField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: route } => todo!(),
        }
        Self {
            entries,
            errors: vec![],
        }
    }
}
