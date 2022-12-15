use husky_absolute_path::AbsolutePath;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubrouteEntry {
    pub ident: Option<RangedIdentifier>,
    pub kind: EntityKind,
}

impl SubrouteEntry {
    pub fn from_token_group(
        db: &dyn EntityTreeDb,
        file: AbsolutePath,
        _parent_entity_kind: EntityKind,
        token_group_index: usize,
        token_group: &[Token],
    ) -> EntityTreeResult<Option<SubrouteEntry>> {
        todo!()
        // if token_group[0].kind == TokenKind::Keyword(Keyword::Use.into()) {
        //     return Ok(None);
        // }
        // if token_group.len() < 2 {
        //     match token_group[0].kind {
        //         TokenKind::Identifier(Identifier::Custom(ident)) => {
        //             return Ok(Some(SubrouteEntry {
        //                 ident: Some(RangedIdentifier {
        //                     ident,
        //                     range: token_group[0].range,
        //                 }),
        //                 kind: EntityKind::EnumVariant,
        //                 source: EntitySource::from_file(file, token_group_index),
        //             }))
        //         }
        //         _ => {
        //             return Err(defn_error!(
        //                 "invalid single token for entity defn head",
        //                 token_group[0].text_range()
        //             ))
        //         }
        //     }
        // }
        // if token_group.len() == 2 {
        //     return match token_group[0].kind {
        //         TokenKind::Keyword(Keyword::Main) => Ok(Some(SubrouteEntry {
        //             ident: None,
        //             kind: EntityKind::Main,
        //             source: EntitySource::from_file(file, token_group_index),
        //         })),
        //         TokenKind::Keyword(Keyword::Mod) => {
        //             SubrouteEntry::submodule(db, file, token_group_index, token_group)
        //         }
        //         _ => Ok(None),
        //     };
        // }
        // match token_group[0].kind {
        //     TokenKind::Keyword(keyword) => match keyword {
        //         Keyword::Paradigm(_) | Keyword::Type(_) | Keyword::Mod => {
        //             if let TokenKind::Identifier(ident) = token_group[1].kind {
        //                 if let Some(kind) = tell_entity_kind(keyword, &token_group[2]) {
        //                     return match ident {
        //                         Identifier::Root(_) => Err(defn_error!(
        //                             "builtin identifiers are reserved",
        //                             token_group[1].text_range()
        //                         )),
        //                         Identifier::Custom(ident) => Ok(Some(SubrouteEntry {
        //                             ident: Some(RangedIdentifier {
        //                                 ident,
        //                                 range: token_group[2].range,
        //                             }),
        //                             kind,
        //                             source: EntitySource::from_file(file, token_group_index),
        //                         })),
        //                         Identifier::Contextual(_) => Err(defn_error!(
        //                             "contextual identifiers are reserved",
        //                             token_group[1].text_range()
        //                         )),
        //                     };
        //                 }
        //             }
        //             Err(defn_error!(
        //                 "second token should be identifier",
        //                 token_group[1].text_range()
        //             ))
        //         }
        //         _ => Ok(None),
        //     },
        //     TokenKind::Decorator(Decorator::Static) => match token_group[1].kind {
        //         TokenKind::Keyword(Keyword::Paradigm(paradigm)) => match token_group[2].kind {
        //             TokenKind::Identifier(Identifier::Custom(ident)) => Ok(Some(SubrouteEntry {
        //                 ident: Some(RangedIdentifier {
        //                     ident,
        //                     range: token_group[2].range,
        //                 }),
        //                 kind: EntityKind::Function {
        //                     requires_lazy: paradigm.is_lazy(),
        //                 },
        //                 source: EntitySource::from_file(file, token_group_index),
        //             })),
        //             _ => todo!(),
        //         },
        //         _ => todo!(),
        //     },
        //     _ => Ok(None),
        // }
    }

    pub fn submodule(
        db: &dyn EntityTreeDb,
        file: AbsolutePath,
        _token_group_index: usize,
        token_group: &[Token],
    ) -> EntityTreeResult<Option<SubrouteEntry>> {
        todo!()
        // let ident = match token_group[1].kind {
        //     TokenKind::Identifier(Identifier::Custom(ident)) => ident,
        //     _ => todo!(),
        // };
        // if let Some(submodule_file) = db.submodule_file(file, ident) {
        //     Ok(Some(SubrouteEntry {
        //         ident: Some(RangedIdentifier {
        //             ident,
        //             range: token_group[1].range,
        //         }),
        //         kind: EntityKind::Module,
        //         source: EntitySource::Module {
        //             file: submodule_file,
        //         },
        //     }))
        // } else {
        //     Err(defn_error!(
        //         format!("file for submodule doesn't exist"),
        //         token_group.text_range()
        //     ))
        // }
    }
}
