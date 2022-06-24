mod entry;

pub use entry::*;

use crate::EntitySyntaxSalsaQueryGroup;
use crate::{error::*, *};
use dev_utils::dev_src;
use entity_kind::MemberKind;
use entity_route::*;
use file::FilePtr;
use print_utils::p;
use static_defn::*;
use text::{RangedCustomIdentifier, TextRange, TextRanged};
use thin_vec::thin_vec;
use token::{SpecialToken, Token, TokenGroupIter, TokenKind};
use word::Identifier;
use word::{CustomIdentifier, Decorator, Keyword};

pub fn tell_entity_kind(keyword: Keyword, third_token: &Token) -> Option<EntityKind> {
    match keyword {
        Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) | Keyword::Liason(_) => None,
        Keyword::Mod => Some(EntityKind::Module),
        Keyword::Paradigm(paradigm) => Some(match third_token.kind {
            TokenKind::Special(SpecialToken::LPar) => EntityKind::Function {
                is_lazy: paradigm.is_lazy(),
            },
            TokenKind::Special(SpecialToken::LightArrow)
            | TokenKind::Special(SpecialToken::Colon) => EntityKind::Feature,
            _ => return None,
        }),
        Keyword::Type(keyword) => Some(EntityKind::Type(keyword.into())),
        Keyword::Main => Some(EntityKind::Main),
        Keyword::Visual => None,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubrouteTable {
    pub route: EntityRoutePtr,
    pub entity_kind: EntityKind,
    pub entries: Vec<SubrouteEntry>,
    pub errors: Vec<EntitySyntaxError>,
}

impl std::fmt::Display for SubrouteTable {
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

impl SubrouteTable {
    pub fn new(route: EntityRoutePtr, entity_kind: EntityKind) -> Self {
        Self {
            route,
            entity_kind,
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(
        db: &dyn EntitySyntaxSalsaQueryGroup,
        file: FilePtr,
        route: EntityRoutePtr,
        entity_kind: EntityKind,
        token_groups: TokenGroupIter,
    ) -> Self {
        let mut errors = Vec::new();
        let entries = token_groups
            .filter_map(|item| {
                match SubrouteEntry::from_token_group(db, file, entity_kind, item.idx, item.value) {
                    Ok(opt_entry) => opt_entry,
                    Err(e) => {
                        errors.push(e);
                        None
                    }
                }
            })
            .collect();
        Self {
            route,
            entity_kind,
            entries,
            errors,
        }
    }

    pub fn submodule_idents(&self) -> Vec<RangedCustomIdentifier> {
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

    pub fn entity_locus(&self, ident: CustomIdentifier) -> EntitySyntaxResult<EntityLocus> {
        query_not_none!(
            self.entries
                .iter()
                .find(|entry| if let Some(entry_ident) = entry.ident {
                    entry_ident.ident == ident
                } else {
                    false
                })
                .map(|entry| entry.source),
            format!("No entity route with ident: \"{}\" among {}", ident, self)
        )
    }

    pub fn entity_kind(&self, ident: CustomIdentifier) -> EntitySyntaxResult<EntityKind> {
        self.entries
            .iter()
            .find(|entry| {
                if let Some(entry_ident) = entry.ident {
                    entry_ident.ident == ident
                } else {
                    false
                }
            })
            .map(|entry| entry.kind)
            .ok_or(query_error!(format!(
                "route `{:?}` has no subroute named `{}`",
                self.route, ident
            )))
    }

    pub fn has_subscope(
        &self,
        ident: CustomIdentifier,
        generic_arguments: &[SpatialArgument],
    ) -> bool {
        if generic_arguments.len() > 0 {
            todo!()
        }
        self.entries
            .iter()
            .find(|entry| {
                if let Some(entry_ident) = entry.ident {
                    entry_ident.ident == ident
                } else {
                    false
                }
            })
            .is_some()
    }

    pub fn entry_iter(&self) -> core::slice::Iter<SubrouteEntry> {
        self.entries.iter()
    }
    pub fn error_iter(&self) -> core::slice::Iter<EntitySyntaxError> {
        self.errors.iter()
    }

    pub fn subroute_iter<'a>(
        &'a self,
        db: &'a dyn EntitySyntaxSalsaQueryGroup,
        parent_route: EntityRoutePtr,
    ) -> impl Iterator<Item = EntityRoutePtr> + 'a {
        self.entries.iter().filter_map(move |entry| {
            entry
                .ident
                .map(|ident| db.make_subroute(parent_route, ident.ident, thin_vec![]))
        })
    }

    pub fn non_module_subroute_iter<'a>(
        &'a self,
        db: &'a dyn EntitySyntaxSalsaQueryGroup,
        parent_route: EntityRoutePtr,
    ) -> impl Iterator<Item = EntityRoutePtr> + 'a {
        self.entries
            .iter()
            .filter_map(move |entry| match entry.kind {
                EntityKind::Module => None,
                _ => entry
                    .ident
                    .map(|ident| db.make_subroute(parent_route, ident.ident, thin_vec![])),
            })
    }

    pub(crate) fn from_static(
        db: &dyn EntitySyntaxSalsaQueryGroup,
        route: EntityRoutePtr,
        entity_kind: EntityKind,
        data: &EntityStaticDefn,
    ) -> Self {
        let mut entries: Vec<SubrouteEntry> = data
            .items
            .iter()
            .map(|data| SubrouteEntry {
                ident: Some(RangedCustomIdentifier {
                    ident: db.intern_word(data.name).opt_custom().unwrap(),
                    range: Default::default(),
                }),
                kind: data.variant.entity_kind(),
                source: (*data).into(),
            })
            .collect();
        match data.variant {
            EntityStaticDefnVariant::Routine { .. } | EntityStaticDefnVariant::Module => (),
            EntityStaticDefnVariant::Model { .. } => todo!(),
            EntityStaticDefnVariant::Ty {
                ty_members: type_members,
                variants,
                ..
            } => {
                for type_member in type_members {
                    entries.push(SubrouteEntry {
                        ident: Some(RangedCustomIdentifier {
                            ident: db.intern_word(type_member.name).custom(),
                            range: Default::default(),
                        }),
                        kind: EntityKind::Member(match type_member.variant {
                            EntityStaticDefnVariant::TyField { .. } => MemberKind::Field,
                            EntityStaticDefnVariant::Method { .. } => {
                                MemberKind::Method { is_lazy: false }
                            }
                            _ => panic!(),
                        }),
                        source: EntityLocus::StaticTypeMember,
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
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: route } => todo!(),
        }
        Self {
            route,
            entity_kind,
            entries,
            errors: vec![],
        }
    }
}
