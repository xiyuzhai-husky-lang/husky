use core::{iter::Peekable, slice::Iter};

use common::*;

use file::FileId;
use folded::FoldedList;
use scope::{GenericArgument, InternScope, Scope, ScopeKind, ScopeRoute};
use text::TextPosition;
use token::{Special, Token, TokenKind, TokenizedText};
use word::CustomIdentifier;

use crate::{
    error::{atom_error, src, AtomErrorKind, Source},
    kind::LambdaHead,
    opr::ListStartAttr,
    query::AtomQuery,
    scope_alias::ScopeAliasStack,
    *,
};

pub struct ScopeLRParser<'a, 'b: 'a> {
    db: &'a dyn AtomQuery,
    iter: &'a mut Peekable<Iter<'b, Token>>,
    pub(super) range: TextRange,
}

impl<'a, 'b: 'a> ScopeLRParser<'a, 'b> {
    pub fn new(
        db: &'a dyn AtomQuery,
        token: &'a Token,
        iter: &'a mut Peekable<Iter<'b, Token>>,
    ) -> Self {
        Self {
            db,
            iter,
            range: token.text_range(),
        }
    }

    fn generic_arguments(&mut self, route: ScopeRoute) -> AtomResult<Vec<GenericArgument>> {
        match route {
            ScopeRoute::Builtin(ident) => match ident {
                BuiltinIdentifier::Void
                | BuiltinIdentifier::I32
                | BuiltinIdentifier::F32
                | BuiltinIdentifier::Debug
                | BuiltinIdentifier::Std
                | BuiltinIdentifier::Core => Ok(Vec::new()),
                BuiltinIdentifier::Fp
                | BuiltinIdentifier::Fn
                | BuiltinIdentifier::FnMut
                | BuiltinIdentifier::FnOnce => self.parse_func_args(),
                BuiltinIdentifier::Vec | BuiltinIdentifier::Array | BuiltinIdentifier::Tuple => {
                    self.parse_angled_args()
                }
            },
            _ => match self.db.scope_kind_from_route(route) {
                ScopeKind::Module | ScopeKind::Value => Ok(Vec::new()),
                ScopeKind::Type | ScopeKind::Trait | ScopeKind::Func => self.parse_angled_args(),
            },
        }
    }

    fn try_eat(&mut self, target: TokenKind) -> bool {
        if let Some(Token { kind, range }) = self.iter.peek() {
            if kind == &target {
                self.range = self.range.start..(range.end);
                self.iter.next();
                return true;
            }
        }
        false
    }

    fn eat(&mut self) -> Option<&'b Token> {
        if let Some(token) = self.iter.next() {
            self.range = self.range.start..(token.text_end());
            Some(token)
        } else {
            None
        }
    }

    fn eat_kind(&mut self, target: TokenKind, src: Source) -> AtomResult<()> {
        if let Some(Token { kind, range }) = self.eat() {
            if target == *kind {
                Ok(())
            } else {
                Err(AtomError {
                    src,
                    range: range.clone(),
                    kind: AtomErrorKind::FailExpectation(format!("{:?}", target)),
                })
            }
        } else {
            Err(AtomError {
                src,
                range: self.range.clone(),
                kind: AtomErrorKind::FailExpectation(format!("{:?} after it", target)),
            })
        }
    }

    fn eat_usize(&mut self, src: Source) -> AtomResult<usize> {
        if let Some(Token {
            kind: TokenKind::I32Literal(i),
            range,
        }) = self.eat()
        {
            if *i < 0 {
                Err(AtomError {
                    range: range.clone(),
                    src,
                    kind: "array size must be nonnegative".into(),
                })
            } else {
                Ok(*i as usize)
            }
        } else {
            Err(AtomError {
                src,
                range: self.range.clone(),
                kind: AtomErrorKind::FailExpectation("custom ident".into()),
            })
        }
    }

    fn eat_i32_literal(&mut self, src: Source) -> AtomResult<i32> {
        if let Some(Token {
            kind: TokenKind::I32Literal(i),
            ..
        }) = self.eat()
        {
            Ok(*i)
        } else {
            Err(AtomError {
                src,
                range: self.range.clone(),
                kind: AtomErrorKind::FailExpectation("custom ident".into()),
            })
        }
    }

    fn eat_custom_ident(&mut self, src: Source) -> AtomResult<CustomIdentifier> {
        if let Some(Token {
            kind: TokenKind::Identifier(Identifier::Custom(ident)),
            ..
        }) = self.eat()
        {
            Ok(*ident)
        } else {
            Err(AtomError {
                src,
                range: self.range.clone(),
                kind: AtomErrorKind::FailExpectation("custom ident".into()),
            })
        }
    }

    fn eat_arg(&mut self, src: Source) -> AtomResult<GenericArgument> {
        if let Some(token) = self.eat() {
            if token.kind == TokenKind::Special(Special::LPar) {
                if self.try_eat(Special::RPar.into()) {
                    if self.try_eat(Special::LightArrow.into()) {
                        let output = self.eat_arg(src!())?;
                        let func_scope = Scope::builtin(word::default_func_type(), vec![output]);
                        return Ok(self.db.intern_scope(func_scope).into());
                    } else {
                        return Ok(ScopeId::Builtin(BuiltinIdentifier::Void).into());
                    }
                }
                let mut args = vec![self.eat_arg(src!())?];
                loop {
                    if self.try_eat(Special::Comma.into()) {
                        if self.try_eat(Special::RPar.into()) {
                            break;
                        }
                        args.push(self.eat_arg(src!())?);
                    } else {
                        self.eat_kind(Special::RPar.into(), src!())?;
                        break;
                    }
                }
                if self.try_eat(Special::LightArrow.into()) {
                    let output = self.eat_arg(src!())?;
                    args.push(output);
                    let func_scope = Scope::builtin(word::default_func_type(), args);
                    return Ok(self.db.intern_scope(func_scope).into());
                } else {
                    let tuple = Scope::builtin(BuiltinIdentifier::Tuple, args);
                    return Ok(self.db.intern_scope(tuple).into());
                }
            } else if let Some((scope, ScopeKind::Type)) = self.try_parse_scope(token)? {
                Ok(scope.into())
            } else {
                Err(AtomError {
                    src,
                    range: token.text_range(),
                    kind: "generic argument".into(),
                })
            }
        } else {
            Err(AtomError {
                src,
                range: self.range.clone(),
                kind: "generic argument after it".into(),
            })
        }
    }

    pub(super) fn try_parse_scope(
        &mut self,
        token: &Token,
    ) -> Result<Option<(ScopeId, ScopeKind)>, AtomError> {
        if token.kind == Special::LBox.into() {
            let scope = if self.try_eat(Special::RBox.into()) {
                let arg = self.eat_arg(src!())?;
                Scope::builtin(BuiltinIdentifier::Vec, vec![arg])
            } else {
                let size: usize = self.eat_usize(src!())?;
                self.eat_kind(Special::RBox.into(), src!())?;
                let arg = self.eat_arg(src!())?;
                Scope::builtin(BuiltinIdentifier::Array, vec![arg, size.into()])
            };
            Ok(Some((self.db.intern_scope(scope), ScopeKind::Type)))
        } else if let Some(route) = self.resolve_scope(token) {
            let mut scope = self.db.intern_scope(Scope {
                generic_arguments: self.generic_arguments(route)?,
                route,
            });
            while self.try_eat(Special::DoubleColon.into()) {
                let ident = self.eat_custom_ident(src!())?;
                let route = ScopeRoute::ChildScope(scope, ident);
                scope = self.db.intern_scope(Scope {
                    route,
                    generic_arguments: self.generic_arguments(route)?,
                });
            }
            Ok(Some((scope, self.db.scope_kind(scope))))
        } else {
            Ok(None)
        }
    }

    fn resolve_scope(&self, token: &Token) -> Option<ScopeRoute> {
        match token.kind {
            TokenKind::Identifier(ident) => match ident {
                Identifier::Builtin(builtin) => Some(builtin.into()),
                Identifier::Custom(_) => todo!(),
            },
            _ => None,
        }
        // ident).ok_or(atom_error!(
        //     &next_token.range,
        //     AtomRule::BeforeColonShouldBeScope,
        // todo!()
    }

    fn resolve_subscope(
        &self,
        parent_scope: Scope,
        subscope_ident: CustomIdentifier,
        start: TextPosition,
        end: TextPosition,
    ) -> Result<Scope, AtomError> {
        self.db
            .subscope(
                self.db.intern_scope(parent_scope),
                subscope_ident,
                Vec::new(),
            )
            .ok_or(atom_error!((start..end).into(), AtomRule::ScopeShouldExist,))
    }

    fn parse_func_args(&mut self) -> Result<Vec<GenericArgument>, AtomError> {
        if self.try_eat(TokenKind::Special(Special::LPar)) {
            let arg0 = self.eat_arg(src!())?;
            let mut args = vec![arg0];
            while self.try_eat(TokenKind::Special(Special::Comma)) {
                if self.try_eat(TokenKind::Special(Special::GreaterOrRAngle)) {
                    return Ok(args);
                }
                args.push(self.eat_arg(src!())?);
            }
            self.eat_kind(Special::RPar.into(), src!())?;
            args.push(if self.try_eat(Special::LightArrow.into()) {
                self.eat_arg(src!())?
            } else {
                ScopeId::Builtin(BuiltinIdentifier::Void).into()
            });
            Ok(args)
        } else {
            Ok(Vec::new())
        }
    }

    fn parse_angled_args(&mut self) -> Result<Vec<GenericArgument>, AtomError> {
        if self.try_eat(TokenKind::Special(Special::LessOrLAngle)) {
            let arg0 = self.eat_arg(src!())?;
            let mut args = vec![arg0];
            while self.try_eat(TokenKind::Special(Special::Comma)) {
                if self.try_eat(TokenKind::Special(Special::GreaterOrRAngle)) {
                    return Ok(args);
                }
                args.push(self.eat_arg(src!())?);
            }
            if let Some(Token {
                kind: TokenKind::Special(Special::GreaterOrRAngle),
                ..
            }) = self.eat()
            {
                Ok(args)
            } else {
                atom_err!(self.range, "matching angular brackets")
            }
        } else {
            Ok(Vec::new())
        }
    }
}
