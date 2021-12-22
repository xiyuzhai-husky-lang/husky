use common::*;

use core::{iter::Peekable, slice::Iter};

use file::FileId;
use folded::FoldedList;
use scope::{BuiltinScope, InternScope, Scope, ScopeKind};
use text::TextPosition;
use token::{Special, Token, TokenKind, TokenizedText};
use word::UserDefinedIdentifier;

use crate::{
    kind::LambdaHead, opr::ListStartAttr, query::AtomQuery, scope_alias::ScopeAliasStack, *,
};

pub struct AtomParser<'a> {
    db: &'a dyn AtomQuery,
    scope_aliases: ScopeAliasStack,
    folded_results: FoldedList<AtomParseResult>,
}

impl<'a> AtomParser<'a> {
    pub(crate) fn new(db: &'a dyn AtomQuery, module: scope::Module) -> Self {
        Self {
            db,
            scope_aliases: ScopeAliasStack::new(),
            folded_results: FoldedList::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<AtomParseResult> {
        self.folded_results
    }

    fn parse_scope(
        &mut self,
        token: &Token,
        iter: &mut Peekable<Iter<Token>>,
    ) -> Result<Option<(ScopeId, ScopeKind, TextPosition)>, AtomError> {
        if let Some(mut scope) = self.resolve_scope(token) {
            scope.generic_arguments = self.parse_generic_arguments(iter)?;
            let mut end = token.text_end();
            while let Some((subscope_ident, new_end)) = parse_coloned_identifier(iter)? {
                end = new_end;
                scope = self.resolve_subscope(scope, subscope_ident, token.text_start(), end)?;
                scope.generic_arguments = self.parse_generic_arguments(iter)?;
            }
            let scope_id = self.db.scope_to_id(scope);
            let scope_kind = self.db.scope_kind(scope_id);
            return Ok(Some((scope_id, scope_kind, end)));
        } else {
            return Ok(None);
        }

        fn identify(token: &Token) -> Result<Identifier, AtomError> {
            match token.kind {
                TokenKind::Identifier(ident) => Ok(ident),
                _ => Err(AtomError::new(
                    token.text_range(),
                    AtomRule::AfterColonShouldBeUserDefinedIdentifier,
                )),
            }
        }

        fn parse_coloned_identifier(
            iter: &mut Peekable<Iter<Token>>,
        ) -> Result<Option<(UserDefinedIdentifier, TextPosition)>, AtomError> {
            if let Some(next_token) = iter.peek().map(|e| *e) {
                if TokenKind::Special(Special::Colon) == next_token.kind {
                    iter.next();
                    let ident_token = iter.next().ok_or(AtomError::new(
                        next_token.text_range(),
                        AtomRule::ScopeShouldExist,
                    ))?;
                    match identify(ident_token)? {
                        Identifier::Builtin(_) => {
                            return Err(AtomError::new(
                                ident_token.text_range(),
                                AtomRule::AfterColonShouldBeUserDefinedIdentifier,
                            ))
                        }
                        Identifier::UserDefined(ident) => {
                            return Ok(Some((ident, ident_token.text_end())))
                        }
                    }
                }
            }
            return Ok(None);
        }
    }

    fn resolve_scope(&self, token: &Token) -> Option<Scope> {
        match token.kind {
            TokenKind::Identifier(ident) => match ident {
                Identifier::Builtin(builtin) => Some(Scope::builtin(builtin, None)),
                Identifier::UserDefined(_) => todo!(),
            },
            _ => None,
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

                let start = token_for_langle.text_start();
                let mut end = token_for_langle.text_end();
                while let Some(token_for_scope) = iter.next() {
                    {
                        let (new_scope, new_scope_kind, new_end) = self
                            .parse_scope(token_for_scope, iter)?
                            .ok_or(AtomError::new(
                                start..end,
                                AtomRule::AfterLAngleShouldBeCommaListOfScopes,
                            ))?;
                        end = new_end;
                        scopes.push(new_scope);
                        ep!(scopes);
                    }

                    {
                        let token_comma_or_rangle = iter.next().ok_or(AtomError::new(
                            start..end,
                            AtomRule::AfterLAngleShouldBeCommaListOfScopes,
                        ))?;
                        end = token_comma_or_rangle.text_end();

                        if token_comma_or_rangle.kind == Special::GreaterOrRAngle.into() {
                            return Ok(Some(scopes));
                        }
                        if token_comma_or_rangle.kind != Special::Comma.into() {
                            epin!();
                            return Err(AtomError::new(start..end, AtomRule::ExpectCommaInAngle));
                        }
                    }
                }
                epin!();
                return Err(AtomError::new(start..end, AtomRule::NonEmptyAngles));
            }
        }
        return Ok(None);
    }

    fn resolve_subscope(
        &self,
        parent_scope: Scope,
        subscope_ident: UserDefinedIdentifier,
        start: TextPosition,
        end: TextPosition,
    ) -> Result<Scope, AtomError> {
        self.db
            .subscope(self.db.scope_to_id(parent_scope), subscope_ident, None)
            .ok_or(AtomError::new(
                (start..end).into(),
                AtomRule::ScopeShouldExist,
            ))
    }
}

impl<'a> folded::Transformer<'_, [Token], TokenizedText, AtomParseResult, AtomParser<'a>>
    for AtomParser<'a>
{
    fn enter_fold(&mut self) {
        self.scope_aliases.enter_fold();
    }

    fn exit_fold(&mut self) {
        self.scope_aliases.exit_fold();
    }

    fn transform(&mut self, token_group: &[Token]) -> AtomParseResult {
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
        let mut expect_lambda_second_vertical = false;

        while let Some(token) = iter.next() {
            if let Some((scope_id, scope_kind, end)) = self.parse_scope(token, &mut iter)? {
                atom_group.push(Atom::new(
                    token.text_range_to(end),
                    AtomKind::Scope(scope_id, scope_kind),
                ))?
            } else {
                match &token.kind {
                    TokenKind::Keyword(_) => {
                        return Err(AtomError::new(
                            token.text_range(),
                            AtomRule::KeywordShouldBeAtStart,
                        ))
                    }
                    TokenKind::Special(special) => match special {
                        Special::DoubleColon => panic!(),
                        Special::Colon => {
                            if None == iter.peek() {
                                return Ok(atom_group);
                            } else {
                                todo!()
                            }
                        }
                        Special::DoubleVertical => {
                            if atom_group.is_convex() {
                                atom_group
                                    .push(Atom::new(token.text_range(), BinaryOpr::BitOr.into()))?;
                            } else {
                                atom_group.push(Atom::new(
                                    token.text_start()..token.text_start().to_right(1),
                                    AtomKind::ListStart(Bracket::Vert, ListStartAttr::None),
                                ))?;
                                atom_group.push(Atom::new(
                                    token.text_end().to_left(1)..token.text_end(),
                                    AtomKind::ListEnd(Bracket::Vert, ListEndAttr::Attach),
                                ))?;
                            }
                        }
                        Special::Vertical => {
                            if expect_lambda_second_vertical {
                                expect_lambda_second_vertical = false;
                                atom_group.end_lambda(token.text_range());
                            } else {
                                expect_lambda_second_vertical = true;
                                atom_group.start_lambda(token.text_range())?;
                            }
                        }
                        Special::Ambersand => {
                            if atom_group.is_convex() {
                                atom_group.push(Atom::new(
                                    token.text_range(),
                                    BinaryOpr::BitAnd.into(),
                                ))?;
                            } else {
                                atom_group.push(Atom::new(
                                    token.text_range(),
                                    PrefixOpr::Shared.into(),
                                ))?;
                            }
                        }
                        Special::Exclamation => todo!(),
                        Special::LPar => atom_group.start_list(Bracket::Par, token.text_range()),
                        Special::LBox => atom_group.start_list(Bracket::Box, token.text_range()),
                        Special::LCurl => atom_group.start_list(Bracket::Curl, token.text_range()),
                        Special::RPar => {
                            epin!();
                            ep!(iter.peek());
                            if let Some(Token {
                                kind: TokenKind::Special(Special::LightArrow),
                                ..
                            }) = iter.peek()
                            {
                                epin!();
                                let arrow_atom = iter.next().unwrap();
                                let no_type_after_arrow_error = AtomError::new(
                                    arrow_atom.text_range(),
                                    AtomRule::ExpectTypeAfterLightArrow,
                                );
                                let (scope_id, scope_kind, end) = self
                                    .parse_scope(
                                        iter.next().ok_or(no_type_after_arrow_error.clone())?,
                                        &mut iter,
                                    )?
                                    .ok_or(no_type_after_arrow_error.clone())?;
                                match scope_kind {
                                    ScopeKind::Type { .. } => Ok(()),
                                    ScopeKind::Value => Err(no_type_after_arrow_error),
                                    ScopeKind::Module => Err(no_type_after_arrow_error),
                                    ScopeKind::Routine { .. } => {
                                        Err(no_type_after_arrow_error.clone())
                                    }
                                    ScopeKind::Trait => Err(no_type_after_arrow_error.clone()),
                                }?;
                                epin!();
                                atom_group.make_default_routine_type(
                                    self.db,
                                    scope_id,
                                    &(arrow_atom.text_start()..end),
                                )?;
                            } else {
                                atom_group.end_list(
                                    Bracket::Par,
                                    ListEndAttr::None,
                                    token.text_range(),
                                )
                            }
                        }
                        Special::RBox => {
                            atom_group.end_list(Bracket::Box, ListEndAttr::None, token.text_range())
                        }
                        Special::RCurl => atom_group.end_list(
                            Bracket::Curl,
                            ListEndAttr::None,
                            token.text_range(),
                        ),
                        // Special::RPar
                        Special::MemberAccess => todo!(),
                        _ => atom_group.push(token.into())?,
                    },
                    _ => atom_group.push(token.into())?,
                }
            }
        }

        Ok(atom_group)
    }

    fn folded_results(&mut self) -> &mut FoldedList<AtomParseResult> {
        &mut self.folded_results
    }
}
