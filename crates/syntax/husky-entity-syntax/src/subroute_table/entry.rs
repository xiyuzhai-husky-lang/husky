use super::*;

#[derive(PartialEq, Eq, Clone)]
pub struct SubrouteEntry {
    pub ident: Option<RangedCustomIdentifier>,
    pub kind: EntityKind,
    pub source: EntitySource,
}

impl std::fmt::Debug for SubrouteEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ident: {:?}, kind: {:?}}}, source: {:?}",
            self.ident, self.kind, self.source
        ))
    }
}

impl SubrouteEntry {
    pub fn from_token_group(
        db: &dyn EntitySyntaxSalsaQueryGroup,
        file: FilePtr,
        parent_entity_kind: EntityKind,
        token_group_index: usize,
        token_group: &[HuskyToken],
    ) -> EntitySyntaxResult<Option<SubrouteEntry>> {
        if token_group[0].kind == HuskyTokenKind::Keyword(Keyword::Use.into()) {
            return Ok(None);
        }
        if token_group.len() < 2 {
            match token_group[0].kind {
                HuskyTokenKind::Identifier(Identifier::Custom(ident)) => {
                    return Ok(Some(SubrouteEntry {
                        ident: Some(RangedCustomIdentifier {
                            ident,
                            range: token_group[0].range,
                        }),
                        kind: EntityKind::EnumVariant,
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
                HuskyTokenKind::Keyword(Keyword::Main) => Ok(Some(SubrouteEntry {
                    ident: None,
                    kind: EntityKind::Main,
                    source: EntitySource::from_file(file, token_group_index),
                })),
                HuskyTokenKind::Keyword(Keyword::Mod) => {
                    SubrouteEntry::submodule(db, file, token_group_index, token_group)
                }
                _ => Ok(None),
            };
        }
        match token_group[0].kind {
            HuskyTokenKind::Keyword(keyword) => match keyword {
                Keyword::Paradigm(_) | Keyword::Type(_) | Keyword::Mod => {
                    if let HuskyTokenKind::Identifier(ident) = token_group[1].kind {
                        if let Some(kind) = tell_entity_kind(keyword, &token_group[2]) {
                            return match ident {
                                Identifier::Root(_) => Err(defn_error!(
                                    "builtin identifiers are reserved",
                                    token_group[1].text_range()
                                )),
                                Identifier::Custom(ident) => Ok(Some(SubrouteEntry {
                                    ident: Some(RangedCustomIdentifier {
                                        ident,
                                        range: token_group[2].range,
                                    }),
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
                _ => Ok(None),
            },
            HuskyTokenKind::Decorator(Decorator::Static) => match token_group[1].kind {
                HuskyTokenKind::Keyword(Keyword::Paradigm(paradigm)) => match token_group[2].kind {
                    HuskyTokenKind::Identifier(Identifier::Custom(ident)) => {
                        Ok(Some(SubrouteEntry {
                            ident: Some(RangedCustomIdentifier {
                                ident,
                                range: token_group[2].range,
                            }),
                            kind: EntityKind::Function {
                                requires_lazy: paradigm.is_lazy(),
                            },
                            source: EntitySource::from_file(file, token_group_index),
                        }))
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
            _ => Ok(None),
        }
    }

    pub fn submodule(
        db: &dyn EntitySyntaxSalsaQueryGroup,
        file: FilePtr,
        token_group_index: usize,
        token_group: &[HuskyToken],
    ) -> EntitySyntaxResult<Option<SubrouteEntry>> {
        let ident = match token_group[1].kind {
            HuskyTokenKind::Identifier(Identifier::Custom(ident)) => ident,
            _ => todo!(),
        };
        if let Some(submodule_file) = db.submodule_file(file, ident) {
            Ok(Some(SubrouteEntry {
                ident: Some(RangedCustomIdentifier {
                    ident,
                    range: token_group[1].range,
                }),
                kind: EntityKind::Module,
                source: EntitySource::Module {
                    file: submodule_file,
                },
            }))
        } else {
            Err(defn_error!(
                format!("file for submodule doesn't exist"),
                token_group.text_range()
            ))
        }
    }
}
