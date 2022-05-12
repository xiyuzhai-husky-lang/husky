use dev_utils::dev_src;
use entity_kind::MemberKind;
use entity_route::*;
use file::{get_submodule_file, FilePtr};
use print_utils::p;
use static_defn::*;
use word::{CustomIdentifier, Keyword};

use crate::EntityRouteSalsaQueryGroup;
use crate::{error::*, *};
use text::{RangedCustomIdentifier, TextRange, TextRanged};
use token::{Special, Token, TokenGroupIter, TokenKind};
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
    ) -> EntitySyntaxResult<Option<Entry>> {
        if token_group[0].kind == TokenKind::Keyword(Keyword::Use.into()) {
            return Ok(None);
        }
        if token_group.len() < 2 {
            match token_group[0].kind {
                TokenKind::Identifier(Identifier::Custom(ident)) => {
                    return Ok(Some(Entry {
                        ident: Some(ident),
                        kind: EntityKind::Literal,
                        source: EntitySource::from_file(file, token_group_index),
                    }))
                }
                _ => {
                    return Err(defn_error!(
                        "invalid single token for entity defn head",
                        token_group[0].text_range()
                    ))
                }
            }
        }
        if token_group.len() == 2 {
            return match token_group[0].kind {
                TokenKind::Keyword(Keyword::Main) => Ok(Some(Entry {
                    ident: None,
                    kind: EntityKind::Routine,
                    source: EntitySource::from_file(file, token_group_index),
                })),
                TokenKind::Keyword(Keyword::Mod) => {
                    Entry::submodule(file, token_group_index, token_group)
                }
                TokenKind::Keyword(Keyword::Config(_)) => Ok(None),
                _ => Err(defn_error!(
                    "invalid tokens for entity defn head",
                    token_group.text_range()
                )),
            };
        }
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                if let TokenKind::Identifier(ident) = token_group[1].kind {
                    if let Some(kind) = tell_entity_kind(keyword, &token_group[2]) {
                        return match ident {
                            Identifier::Builtin(_) => Err(defn_error!(
                                "builtin identifiers are reserved",
                                token_group[1].text_range()
                            )),
                            Identifier::Custom(user_defined_ident) => Ok(Some(Entry {
                                ident: Some(user_defined_ident),
                                kind,
                                source: EntitySource::from_file(file, token_group_index),
                            })),
                            Identifier::Contextual(_) => Err(defn_error!(
                                "contextual identifiers are reserved",
                                token_group[1].text_range()
                            )),
                        };
                    }
                }
                Err(defn_error!(
                    "second token should be identifier",
                    token_group[1].text_range()
                ))
            }
            _ => Err(defn_error!(
                "first token should be identifier",
                token_group[0].text_range()
            )),
        }
    }

    pub fn submodule(
        file: FilePtr,
        token_group_index: usize,
        token_group: &[Token],
    ) -> EntitySyntaxResult<Option<Entry>> {
        let ident = match token_group[1].kind {
            TokenKind::Identifier(Identifier::Custom(ident)) => ident,
            _ => todo!(),
        };
        if get_submodule_file(&file, ident).is_none() {
            return Err(defn_error!(
                format!("file for submodule doesn't exist"),
                token_group.text_range()
            ));
        }
        return Ok(Some(Entry {
            ident: Some(ident),
            kind: EntityKind::Module,
            source: EntitySource::WithinModule {
                file,
                token_group_index,
            },
        }));
    }
}

pub fn tell_entity_kind(keyword: Keyword, third_token: &Token) -> Option<EntityKind> {
    match keyword {
        Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) => None,
        Keyword::Mod => Some(EntityKind::Module),
        Keyword::Routine(_) => Some(EntityKind::Routine),
        Keyword::Type(keyword) => Some(EntityKind::Type(keyword.into())),
        Keyword::Def => Some(match third_token.kind {
            TokenKind::Special(Special::LCurl) => EntityKind::Pattern,
            _ => EntityKind::Feature,
        }),
        Keyword::Main => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChildRouteTable {
    pub entries: Vec<Entry>,
    pub errors: Vec<EntitySyntaxError>,
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
            .filter_map(
                |item| match Entry::from_token_group(file_id, item.idx, item.value) {
                    Ok(opt_entry) => opt_entry,
                    Err(e) => {
                        errors.push(e);
                        None
                    }
                },
            )
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

    pub fn entity_source(&self, ident: CustomIdentifier) -> EntitySyntaxResult<EntitySource> {
        query_not_none!(
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
    pub fn error_iter(&self) -> core::slice::Iter<EntitySyntaxError> {
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
                        kind: EntityKind::Member(match type_member.variant {
                            EntityStaticDefnVariant::TypeField { .. } => MemberKind::Field,
                            EntityStaticDefnVariant::Method { .. } => MemberKind::Method,
                            _ => panic!(),
                        }),
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
