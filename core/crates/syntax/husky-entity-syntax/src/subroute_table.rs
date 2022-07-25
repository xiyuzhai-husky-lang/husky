mod entry;

pub use entry::*;

use crate::{error::*, *};
use entity_kind::MemberKind;
use husky_dev_utils::dev_src;
use husky_entity_route::*;
use husky_file::FilePtr;
use husky_print_utils::p;
use husky_text::{RangedCustomIdentifier, TextRange, TextRanged};
use husky_token::{HuskyToken, HuskyTokenKind, SpecialToken, TokenGroupIter};
use husky_word::Identifier;
use husky_word::{CustomIdentifier, Decorator, Keyword};
use static_defn::*;
use thin_vec::thin_vec;

pub fn tell_entity_kind(keyword: Keyword, third_token: &HuskyToken) -> Option<EntityKind> {
    match keyword {
        Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) | Keyword::Liason(_) => None,
        Keyword::Mod => Some(EntityKind::Module),
        Keyword::Paradigm(paradigm) => Some(match third_token.kind {
            HuskyTokenKind::Special(SpecialToken::LPar) => EntityKind::Function {
                requires_lazy: paradigm.is_lazy(),
            },
            HuskyTokenKind::Special(SpecialToken::LightArrow)
            | HuskyTokenKind::Special(SpecialToken::Colon) => EntityKind::Feature,
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
            .map(|entry| {
                f.write_fmt(format_args!(
                    "{},",
                    entry.ident.map(|ident| ident.ident.as_str()).unwrap_or("_")
                ))
            })
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

    pub fn entity_source(&self, ident: CustomIdentifier) -> EntitySyntaxResult<EntitySource> {
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
                .map(|ident| db.subroute(parent_route, ident.ident, thin_vec![]))
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
                    .map(|ident| db.subroute(parent_route, ident.ident, thin_vec![])),
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
            EntityStaticDefnVariant::Function { .. } | EntityStaticDefnVariant::Module => (),
            EntityStaticDefnVariant::Ty {
                ty_members,
                variants,
                ..
            } => {
                for ty_member in ty_members {
                    entries.push(SubrouteEntry {
                        ident: Some(RangedCustomIdentifier {
                            ident: db.intern_word(ty_member.name).custom(),
                            range: Default::default(),
                        }),
                        kind: EntityKind::Member(match ty_member.variant {
                            EntityStaticDefnVariant::TyField { .. } => MemberKind::Field,
                            EntityStaticDefnVariant::Method { .. } => {
                                MemberKind::Method { is_lazy: false }
                            }
                            _ => panic!(),
                        }),
                        source: EntitySource::StaticTypeMember(ty_member),
                    })
                }
                for variant in variants {
                    todo!()
                }
            }
            EntityStaticDefnVariant::Trait {
                base_route,
                spatial_parameters,
                ref members,
            } => {
                for member in members.iter() {
                    entries.push(SubrouteEntry {
                        ident: Some(RangedCustomIdentifier {
                            ident: db.intern_word(member.name).custom(),
                            range: Default::default(),
                        }),
                        kind: EntityKind::Member(match member.variant {
                            EntityStaticDefnVariant::Method { .. } => {
                                MemberKind::Method { is_lazy: false }
                            }
                            EntityStaticDefnVariant::TraitAssociatedType { trai, traits } => {
                                MemberKind::TraitAssociatedType
                            }
                            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
                            _ => panic!(),
                        }),
                        source: EntitySource::StaticTraitMember(member),
                    })
                }
            }
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
