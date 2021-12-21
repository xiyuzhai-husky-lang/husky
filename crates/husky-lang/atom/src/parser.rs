use core::{iter::Peekable, slice::Iter};

use folded::FoldedList;
use scope::BuiltinScope;
use text::TextPosition;
use token::{Special, Token, TokenKind, TokenizedText};

use crate::{
    kind::LambdaHead, opr::ListStartAttr, query::AtomQuery, scope_alias::ScopeAliasStack, *,
};

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
        if let Some(mut scope) = self.resolve_scope(token) {
            let mut end = token.text_end();
            if let Some((subscope_ident, end0)) = parse_coloned_identifier(iter)? {
                end = end0;
                let start = token.text_start();
                let generic_arguments = self.parse_generic_arguments(iter)?;
                scope = self.subscope(scope, subscope_ident, generic_arguments, start, end0)?;
                while let Some((subscope_ident, end1)) = parse_coloned_identifier(iter)? {
                    end = end1;
                    let raw_scope = self.subscope(scope, subscope_ident, None, start, end)?;
                    if self.db.is_scope_generic(raw_scope) {
                        let generic_arguments = self.parse_generic_arguments(iter)?;
                        scope =
                            self.subscope(scope, subscope_ident, generic_arguments, start, end)?;
                    } else {
                        scope = raw_scope;
                    }
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
                    token.text_range(),
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
                    let ident_token = iter.next().ok_or(AtomError::new(
                        next_token.text_range(),
                        AtomRule::ScopeShouldExist,
                    ))?;
                    return Ok(Some((identify(ident_token)?, ident_token.text_end())));
                }
            }
            return Ok(None);
        }
    }

    fn resolve_scope(&self, token: &Token) -> Option<ScopeId> {
        match token.kind {
            TokenKind::Identifier(ident) => match ident {
                Identifier::Reserved(reserved) => Some(match reserved {
                    word::Reserved::Std => BuiltinScope::Std.into(),
                    word::Reserved::Core => BuiltinScope::Core.into(),
                    word::Reserved::Debug => BuiltinScope::Debug.into(),
                    word::Reserved::I32 => BuiltinScope::I32.into(),
                    word::Reserved::F32 => BuiltinScope::F32.into(),
                    word::Reserved::Vec => BuiltinScope::Vec.into(),
                    word::Reserved::Tuple => BuiltinScope::Tuple.into(),
                    word::Reserved::Fp => BuiltinScope::Fp.into(),
                    word::Reserved::Fn => BuiltinScope::Fn.into(),
                    word::Reserved::FnMut => BuiltinScope::FnMut.into(),
                    word::Reserved::FnOnce => BuiltinScope::FnOnce.into(),
                }),
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
                        end = token_comma_or_rangle.text_end();

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

impl<'a> folded::Transformer<'_, [Token], TokenizedText, AtomResult, AtomParser<'a>>
    for AtomParser<'a>
{
    fn enter_fold(&mut self) {
        self.scope_aliases.enter_fold();
    }

    fn exit_fold(&mut self) {
        self.scope_aliases.exit_fold();
    }

    fn transform(&mut self, token_group: &[Token]) -> AtomResult {
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
            if let Some((scope, end)) = self.parse_scope(token, &mut iter)? {
                atom_group.push(Atom::new(token.text_range_to(end), AtomKind::Scope(scope)))?
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
                                // atom_group
                                //     .push(Atom::new(token.range, BinaryOpr::WithType.into()))?;
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
                            if let Some(Token {
                                kind: TokenKind::Special(Special::LightArrow),
                                ..
                            }) = iter.peek()
                            {
                                let arrow_atom = iter.next().unwrap();
                                let no_type_after_arrow_error = AtomError::new(
                                    arrow_atom.text_range(),
                                    AtomRule::ExpectTypeAfterLightArrow,
                                );
                                let (scope, end) = self
                                    .parse_scope(
                                        iter.next().ok_or(no_type_after_arrow_error.clone())?,
                                        &mut iter,
                                    )?
                                    .ok_or(no_type_after_arrow_error.clone())?;
                                match self.db.scope_kind(scope) {
                                    scope::ScopeKind::Type { .. } => Ok(()),
                                    scope::ScopeKind::Value => Err(no_type_after_arrow_error),
                                    scope::ScopeKind::Module => Err(no_type_after_arrow_error),
                                    scope::ScopeKind::Routine { .. } => {
                                        Err(no_type_after_arrow_error.clone())
                                    }
                                }?;
                                atom_group.make_default_func_type(
                                    self.db,
                                    scope,
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

    fn folded_results(&mut self) -> &mut FoldedList<AtomResult> {
        &mut self.folded_results
    }
}
