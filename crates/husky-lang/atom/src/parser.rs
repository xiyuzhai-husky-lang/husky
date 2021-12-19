use core::{iter::Peekable, slice::Iter};

use folded::FoldedList;
use text::TextPosition;
use token::{Special, Token, TokenKind, TokenizedText};

use crate::{query::AtomQuery, scope_alias::ScopeAliasStack, *};

pub struct AtomParser<'a> {
    db: &'a dyn AtomQuery,
    scope_aliases: ScopeAliasStack,
    folded_results: FoldedList<AtomResult>,
}

impl<'a> AtomParser<'a> {
    pub(crate) fn new(db: &'a dyn AtomQuery) -> Self {
        Self {
            db,
            scope_aliases: ScopeAliasStack::new(),
            folded_results: FoldedList::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<AtomResult> {
        self.folded_results
    }

    fn parse_scope(
        &mut self,
        token: &Token,
        iter: &mut Peekable<Iter<Token>>,
    ) -> Result<Option<(ScopeId, TextPosition)>, AtomError> {
        if let Some((subscope_ident, end0)) = parse_coloned_identifier(iter)? {
            let mut scope = self.resolve_scope(token)?;
            let mut end = end0;
            let start = token.range.start;
            let generic_arguments = self.parse_generic_arguments(iter)?;
            scope = self.subscope(scope, subscope_ident, generic_arguments, start, end0)?;
            while let Some((subscope_ident, end1)) = parse_coloned_identifier(iter)? {
                end = end1;
                let raw_scope = self.subscope(scope, subscope_ident, None, start, end)?;
                if self.db.is_scope_generic(raw_scope) {
                    let generic_arguments = self.parse_generic_arguments(iter)?;
                    scope = self.subscope(scope, subscope_ident, generic_arguments, start, end)?;
                } else {
                    scope = raw_scope;
                }
            }
            return Ok(Some((scope, end)));
        } else {
            return Ok(None);
        }

        fn identify(token: &Token) -> Result<Identifier, AtomError> {
            match token.kind {
                TokenKind::Identifier(ident) => Ok(ident),
                _ => Err(AtomError::new(
                    token.range,
                    AtomRule::AfterColonShouldBeIdentifier,
                )),
            }
        }

        fn parse_coloned_identifier(
            iter: &mut Peekable<Iter<Token>>,
        ) -> Result<Option<(Identifier, TextPosition)>, AtomError> {
            if let Some(next_token) = iter.peek().map(|e| *e) {
                if TokenKind::Special(Special::Colon) == next_token.kind {
                    iter.next();
                    let ident_token = iter
                        .next()
                        .ok_or(AtomError::new(next_token.range, AtomRule::ScopeShouldExist))?;
                    return Ok(Some((identify(ident_token)?, ident_token.range.end)));
                }
            }
            return Ok(None);
        }
    }

    fn resolve_scope(&self, token: &Token) -> Result<ScopeId, AtomError> {
        match token.kind {
            TokenKind::Identifier(ident) => todo!(),
            _ => todo!(),
        }
        // ident).ok_or(AtomError::new(
        //     &next_token.range,
        //     AtomRule::BeforeColonShouldBeScope,
        // todo!()
    }

    fn parse_generic_arguments(
        &mut self,
        iter: &mut Peekable<Iter<Token>>,
    ) -> Result<Option<Vec<ScopeId>>, AtomError> {
        if let Some(token_for_langle) = iter.peek().map(|e| *e) {
            if token_for_langle.kind == TokenKind::Special(Special::LessOrLAngle) {
                let mut scopes = Vec::new();
                iter.next();

                let start = token_for_langle.range.start;
                let mut end = token_for_langle.range.end;
                while let Some(token_for_scope) = iter.next() {
                    {
                        let (new_scope, new_end) =
                            self.parse_scope(token_for_scope, iter)?
                                .ok_or(AtomError::new(
                                    (start..end).into(),
                                    AtomRule::AfterLAngleShouldBeCommaListOfScopes,
                                ))?;
                        end = new_end;
                        scopes.push(new_scope);
                    }

                    {
                        let token_comma_or_rangle = iter.next().ok_or(AtomError::new(
                            (start..end).into(),
                            AtomRule::AfterLAngleShouldBeCommaListOfScopes,
                        ))?;
                        end = token_comma_or_rangle.range.end;

                        if token_comma_or_rangle.kind == TokenKind::Special(Special::Comma) {
                            if scopes.len() == 0 {
                                return Err(AtomError::new(
                                    (start..end).into(),
                                    AtomRule::GenericArgumentsShouldBeNonEmpty,
                                ));
                            }
                            return Ok(Some(scopes));
                        } else if token_comma_or_rangle.kind
                            != TokenKind::Special(Special::GreaterOrRAngle)
                        {
                            return Err(AtomError::new(
                                (start..end).into(),
                                AtomRule::GenericArgumentsShouldBeNonEmpty,
                            ));
                        }
                    }
                }
                return Err(AtomError::new(
                    (start..end).into(),
                    AtomRule::GenericArgumentsShouldBeNonEmpty,
                ));
            }
        }
        return Ok(None);
    }

    fn subscope(
        &self,
        parent_scope: ScopeId,
        subscope_ident: Identifier,
        generic_arguments: Option<Vec<ScopeId>>,
        start: TextPosition,
        end: TextPosition,
    ) -> Result<ScopeId, AtomError> {
        self.db
            .subscope(parent_scope, subscope_ident, generic_arguments)
            .ok_or(AtomError::new(
                (start..end).into(),
                AtomRule::ScopeShouldExist,
            ))
    }
}

impl<'a> folded::Parser<'_, [Token], TokenizedText, AtomResult, AtomParser<'a>> for AtomParser<'a> {
    fn enter_fold(&mut self) {
        self.scope_aliases.enter_fold();
    }

    fn exit_fold(&mut self) {
        self.scope_aliases.exit_fold();
    }

    fn parse(&mut self, token_group: &[Token]) -> AtomResult {
        let mut iter = token_group.iter().peekable();
        let mut atom_group = AtomGroup::new(
            if let TokenKind::Keyword(keyword) = iter.peek().unwrap().kind {
                iter.next();
                Some(keyword)
            } else {
                None
            },
            token_group.last().unwrap().kind == TokenKind::Special(Special::Colon),
        );

        while let Some(token) = iter.next() {
            if let Some((scope, end)) = self.parse_scope(token, &mut iter)? {
                atom_group.push(Atom::new(
                    (token.range.start..end).into(),
                    AtomKind::Scope(scope),
                ))?
            } else {
                match &token.kind {
                    TokenKind::Keyword(_) => {
                        return Err(AtomError::new(
                            token.range,
                            AtomRule::KeywordShouldBeAtStart,
                        ))
                    }
                    TokenKind::Special(special) => match special {
                        token::Special::DoubleColon => panic!(),
                        token::Special::DoubleVertical => {
                            if atom_group.is_convex() {
                                atom_group.push(Atom::new(token.range, BinaryOpr::BitOr.into()))?;
                            } else {
                                atom_group
                                    .push(Atom::new(token.range, PrefixOpr::LambdaBegin.into()))?;
                                atom_group
                                    .push(Atom::new(token.range, BinaryOpr::LambdaMiddle.into()))?;
                            }
                        }
                        token::Special::Colon => {
                            if None == iter.peek() {
                                return Ok(atom_group);
                            } else {
                                todo!()
                                // atom_group
                                //     .push(Atom::new(token.range, BinaryOpr::WithType.into()))?;
                            }
                        }
                        token::Special::Vertical => {
                            if atom_group.is_convex() {
                                atom_group
                                    .push(Atom::new(token.range, BinaryOpr::LambdaMiddle.into()))?;
                                todo!();
                            } else {
                                atom_group
                                    .push(Atom::new(token.range, PrefixOpr::LambdaBegin.into()))?;
                            }
                        }
                        token::Special::Ambersand => {
                            if atom_group.is_convex() {
                                atom_group
                                    .push(Atom::new(token.range, BinaryOpr::BitAnd.into()))?;
                            } else {
                                atom_group
                                    .push(Atom::new(token.range, PrefixOpr::Shared.into()))?;
                            }
                        }
                        token::Special::Exclamation => todo!(),
                        token::Special::LPar => todo!(),
                        token::Special::LBox => {
                            if atom_group.is_convex() {
                                atom_group.push(Atom::new(
                                    (token.range.start..token.range.start).into(),
                                    BinaryOpr::Index.into(),
                                ))?;
                                atom_group.push(token.into())?;
                            } else {
                                todo!()
                            }
                        }
                        token::Special::LCurl => todo!(),
                        token::Special::MemberAccess => todo!(),
                        _ => atom_group.push(token.into())?,
                    },
                    _ => atom_group.push(token.into())?,
                }
            }
        }

        Ok(atom_group)
    }

    fn folded_results(&mut self) -> &mut FoldedList<AtomResult> {
        &mut self.folded_results
    }
}
