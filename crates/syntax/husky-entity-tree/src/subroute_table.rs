mod entry;

pub use entry::*;
use husky_entity_path::EntityPath;
use husky_opn_syntax::{BinaryOpr, Bracket};

use crate::{error::*, *};
use husky_dev_utils::dev_src;
use husky_entity_kind::{EntityKind, MemberKind};
use husky_term::*;
use husky_text::{HasTextRange, RangedIdentifier};
use husky_token::{Keyword, SpecialToken, Token, TokenKind};
use husky_word::Identifier;
use thin_vec::thin_vec;

pub fn tell_entity_kind(keyword: Keyword, third_token: &Token) -> Option<EntityKind> {
    todo!()
    // match keyword {
    //     Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) | Keyword::Liason(_) => None,
    //     Keyword::Mod => Some(EntityKind::Module),
    //     Keyword::Paradigm(paradigm) => Some(match third_token.kind {
    //         TokenKind::Special(SpecialToken::Bra(Bracket::Par)) => EntityKind::Function {
    //             requires_lazy: paradigm.is_lazy(),
    //         },
    //         TokenKind::Special(SpecialToken::BinaryOpr(BinaryOpr::Curry))
    //         | TokenKind::Special(SpecialToken::Colon) => EntityKind::Feature,
    //         _ => return None,
    //     }),
    //     Keyword::Type(keyword) => Some(EntityKind::Type(keyword.into())),
    //     Keyword::Main => Some(EntityKind::Main),
    //     Keyword::Visual => None,
    // }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubrouteTable {
    pub entity_path: EntityPath,
    pub husky_entity_kind: EntityKind,
    pub entries: Vec<SubrouteEntry>,
    pub errors: Vec<EntityTreeError>,
}

impl SubrouteTable {
    pub fn new(entity_path: EntityPath, husky_entity_kind: EntityKind) -> Self {
        Self {
            entity_path,
            husky_entity_kind,
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    // pub fn parse(
    //     db: &dyn EntityTreeDb,
    //     file: AbsolutePath,
    //     entity_path: EntityPath,
    //     husky_entity_kind: EntityKind,
    //     token_groups: TokenGroupIter,
    // ) -> Self {
    //     let mut errors = Vec::new();
    //     let entries = token_groups
    //         .filter_map(|item| {
    //             match SubrouteEntry::from_token_group(
    //                 db,
    //                 file,
    //                 husky_entity_kind,
    //                 item.idx,
    //                 item.value,
    //             ) {
    //                 Ok(opt_entry) => opt_entry,
    //                 Err(e) => {
    //                     errors.push(e);
    //                     None
    //                 }
    //             }
    //         })
    //         .collect();
    //     Self {
    //         entity_path,
    //         husky_entity_kind,
    //         entries,
    //         errors,
    //     }
    // }

    pub fn submodule_idents(&self) -> Vec<RangedIdentifier> {
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

    // pub fn entity_source(&self, ident: Identifier) -> EntityTreeResult<EntitySource> {
    //     todo!()
    //     // query_not_none!(
    //     //     self.entries
    //     //         .iter()
    //     //         .find(|entry| if let Some(entry_ident) = entry.ident {
    //     //             entry_ident.ident == ident
    //     //         } else {
    //     //             false
    //     //         })
    //     //         .map(|entry| entry.source),
    //     //     format!("No entity route with ident: \"{}\" among {}", ident, self)
    //     // )
    // }

    pub fn husky_entity_kind(&self, ident: Identifier) -> EntityTreeResult<EntityKind> {
        todo!()
        // self.entries
        //     .iter()
        //     .find(|entry| {
        //         if let Some(entry_ident) = entry.ident {
        //             entry_ident.ident == ident
        //         } else {
        //             false
        //         }
        //     })
        //     .map(|entry| entry.kind)
        //     .ok_or(query_error!(format!(
        //         "route `{:?}` has no subroute named `{}`",
        //         self.route, ident
        //     )))
    }

    // pub fn has_subscope(
    //     &self,
    //     ident: Identifier,
    //     generic_arguments: &[SpatialArgument],
    // ) -> bool {
    //     if generic_arguments.len() > 0 {
    //         todo!()
    //     }
    //     self.entries
    //         .iter()
    //         .find(|entry| {
    //             if let Some(entry_ident) = entry.ident {
    //                 entry_ident.ident == ident
    //             } else {
    //                 false
    //             }
    //         })
    //         .is_some()
    // }

    pub fn entry_iter(&self) -> core::slice::Iter<SubrouteEntry> {
        self.entries.iter()
    }
    pub fn error_iter(&self) -> core::slice::Iter<EntityTreeError> {
        self.errors.iter()
    }

    // pub fn subroute_iter<'a>(
    //     &'a self,
    //     db: &'a dyn EntityTreeDb,
    //     parent: EntityPath,
    // ) -> impl Iterator<Item = Term> + 'a {
    //     self.entries.iter().filter_map(move |entry| {
    //         entry.ident.map(
    //             |ident| todo!(), //  db.subroute(parent, ident.ident, thin_vec![])
    //         )
    //     })
    // }

    // pub fn subentity_kinded_route_iter<'a>(
    //     &'a self,
    //     db: &'a dyn EntityTreeDb,
    //     parent_route: Term,
    // ) -> impl Iterator<Item = (EntityKind, Term)> + 'a {
    //     self.entries.iter().filter_map(move |entry| {
    //         entry.ident.map(|ident| {
    //             (
    //                 entry.kind,
    //                 todo!(), // db.subroute(parent_route, ident.ident, thin_vec![]),
    //             )
    //         })
    //     })
    // }

    // pub fn submodule_route_iter<'a>(
    //     &'a self,
    //     db: &'a dyn EntityTreeDb,
    //     parent_route: Term,
    // ) -> impl Iterator<Item = Term> + 'a {
    //     self.entries
    //         .iter()
    //         .filter_map(move |entry| match entry.kind {
    //             EntityKind::Module => entry
    //                 .ident
    //                 .map(|ident| db.subroute(parent_route, ident.ident, thin_vec![])),
    //             _ => None,
    //         })
    // }

    // pub fn non_module_subroute_iter<'a>(
    //     &'a self,
    //     db: &'a dyn EntityTreeDb,
    //     parent_route: Term,
    // ) -> impl Iterator<Item = Term> + 'a {
    //     self.entries
    //         .iter()
    //         .filter_map(move |entry| match entry.kind {
    //             EntityKind::Module => None,
    //             _ => entry
    //                 .ident
    //                 .map(|ident| db.subroute(parent_route, ident.ident, thin_vec![])),
    //         })
    // }

    // pub(crate) fn from_static(
    //     db: &dyn EntityTreeDb,
    //     entity_path: EntityPath,
    //     husky_entity_kind: EntityKind,
    //     data: &EntityStaticDefn,
    // ) -> Self {
    //     todo!()
    //     // let mut entries: Vec<SubrouteEntry> = data
    //     //     .items
    //     //     .iter()
    //     //     .map(|data| SubrouteEntry {
    //     //         ident: Some(RangedIdentifier {
    //     //             ident: db.it_word(data.name).opt_custom().unwrap(),
    //     //             range: Default::default(),
    //     //         }),
    //     //         kind: data.variant.husky_entity_kind(),
    //     //         source: (*data).into(),
    //     //     })
    //     //     .collect();
    //     // match data.variant {
    //     //     EntityStaticDefnVariant::Function { .. } | EntityStaticDefnVariant::Module => (),
    //     //     EntityStaticDefnVariant::Term {
    //     //         ty_members,
    //     //         variants,
    //     //         ..
    //     //     } => {
    //     //         for ty_member in ty_members {
    //     //             entries.push(SubrouteEntry {
    //     //                 ident: Some(RangedIdentifier {
    //     //                     ident: db.it_word(ty_member.name).custom(),
    //     //                     range: Default::default(),
    //     //                 }),
    //     //                 kind: EntityKind::Member(match ty_member.variant {
    //     //                     EntityStaticDefnVariant::TyField { .. } => MemberKind::Field,
    //     //                     EntityStaticDefnVariant::Method { .. } => {
    //     //                         MemberKind::Method { is_lazy: false }
    //     //                     }
    //     //                     _ => panic!(),
    //     //                 }),
    //     //                 source: EntitySource::StaticTypeMember(ty_member),
    //     //             })
    //     //         }
    //     //         for variant in variants {
    //     //             entries.push(SubrouteEntry {
    //     //                 ident: Some(RangedIdentifier {
    //     //                     ident: db.custom_ident(variant.name),
    //     //                     range: Default::default(),
    //     //                 }),
    //     //                 kind: EntityKind::EnumVariant,
    //     //                 source: EntitySource::StaticEnumVariant(variant),
    //     //             })
    //     //         }
    //     //     }
    //     //     EntityStaticDefnVariant::Trait { ref members, .. } => {
    //     //         for member in members.iter() {
    //     //             entries.push(SubrouteEntry {
    //     //                 ident: Some(RangedIdentifier {
    //     //                     ident: db.it_word(member.name).custom(),
    //     //                     range: Default::default(),
    //     //                 }),
    //     //                 kind: EntityKind::Member(match member.variant {
    //     //                     EntityStaticDefnVariant::Method { .. } => {
    //     //                         MemberKind::Method { is_lazy: false }
    //     //                     }
    //     //                     EntityStaticDefnVariant::TraitAssociatedType { .. } => {
    //     //                         MemberKind::TraitAssociatedType
    //     //                     }
    //     //                     EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
    //     //                     _ => panic!(),
    //     //                 }),
    //     //                 source: EntitySource::StaticTraitMember(member),
    //     //             })
    //     //         }
    //     //     }
    //     //     EntityStaticDefnVariant::Method { .. } => todo!(),
    //     //     EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
    //     //     EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
    //     //     EntityStaticDefnVariant::TyField { .. } => todo!(),
    //     //     EntityStaticDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
    //     //     EntityStaticDefnVariant::EnumVariant => todo!(),
    //     // }
    //     // Self {
    //     //     entity_path: todo!(),
    //     //     husky_entity_kind,
    //     //     entries,
    //     //     errors: vec![],
    //     // }
    // }
}
