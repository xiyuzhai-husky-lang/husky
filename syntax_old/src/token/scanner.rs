use crate::token::input::TokenGroupCharStream;
use crate::token::output::TokenStream;
use crate::token::receiver::TokenReceiver;
use crate::token::*;
use common::{file, ParserError, ParserErrorVariant, SyntaxError};
pub struct TokenScanner<'lex> {
    sess: &'lex mut Session,
    stream: &'lex mut TokenGroupCharStream<'lex>,
    j_start: usize,
    tokens: TokenReceiver,
}
impl<'lex> TokenScanner<'lex> {
    pub fn new(
        sess: &'lex mut Session,
        stream: &'lex mut TokenGroupCharStream<'lex>,
    ) -> TokenScanner<'lex> {
        let mut scanner = TokenScanner::<'lex> {
            sess,
            stream,
            j_start: 0,
            tokens: TokenReceiver::new(),
        };
        scanner.skip_spaces();
        scanner
    }
    pub fn output(self) -> TokenStream {
        self.tokens.output()
    }
    fn skip_spaces(&mut self) {
        while !self.is_end() && self.stream.top() == ' ' {
            self.stream.pass();
        }
        self.j_start = self.stream.get_j();
    }
    fn is_end(&self) -> bool {
        self.stream.is_end()
    }
    fn top(&self) -> char {
        self.stream.top()
    }
    fn pass(&mut self) {
        self.stream.pass()
    }
    fn token_value(&self) -> &'lex str {
        self.stream.token_value_from(self.j_start)
    }
    fn is_top_digit(&self) -> bool {
        self.top().is_digit(10)
    }
    fn is_top_alphabetic_or_underline(&self) -> bool {
        return self.top().is_alphabetic() || self.top() == '_';
    }
    fn is_top_special(&self) -> bool {
        const SPECIAL_CHARS: &str = "^|!~%&+-*/=><{}[]():.,";
        match SPECIAL_CHARS.find(self.top()) {
            None => false,
            Some(_) => true,
        }
    }
    fn get_range(&self) -> file::Range {
        file::Range {
            start: file::Position {
                i: self.stream.get_i(),
                j: self.j_start,
            },
            end: file::Position {
                i: self.stream.get_i(),
                j: self.stream.get_j(),
            },
        }
    }
    pub fn lex_tokens(&mut self) -> Result<(), ParserError> {
        while !self.is_end() {
            if self.is_top_digit() {
                self.lex_number()?
            } else if self.is_top_alphabetic_or_underline() {
                self.lex_word()?
            } else if self.is_top_special() {
                self.lex_special()?
            } else {
                todo!();
            }
            self.skip_spaces();
        }
        assert!(self.tokens.len() >= 2);
        Ok(())
    }
    fn lex_number(&mut self) -> Result<(), ParserError> {
        while self.is_top_digit() {
            self.pass();
        }
        if self.top() == '.' {
            while self.is_top_digit() {
                self.pass();
            }
            if self.top() == 'e' {
                todo!();
                // err("TODO: scientific");
            }
            self.add_token(TokenVariant::Atom(Atom::Float(
                self.token_value().parse::<f32>().unwrap(),
            )))
        } else {
            self.add_token(TokenVariant::Atom(Atom::Int(
                self.token_value().parse::<i32>().unwrap(),
            )))
        }
    }
    fn lex_word(&mut self) -> Result<(), ParserError> {
        while self.is_top_alphabetic_or_underline() || self.is_top_digit() || self.top() == '_' {
            self.pass();
        }
        fn get_symbol(value: &str) -> Option<(TokenVariant, Precedence)> {
            return match value {
                "use" => Some((TokenVariant::Keyword(Keyword::Use), Precedence::Inert)),
                "pub" => Some((TokenVariant::Keyword(Keyword::Pub), Precedence::Inert)),
                "mod" => Some((TokenVariant::Keyword(Keyword::Mod), Precedence::Inert)),
                "incl" => Some((TokenVariant::Keyword(Keyword::Include), Precedence::Inert)),
                "def" => Some((TokenVariant::Keyword(Keyword::Def), Precedence::Inert)),
                "func" => Some((TokenVariant::Keyword(Keyword::Func), Precedence::Inert)),
                "struct" => Some((TokenVariant::Keyword(Keyword::Struct), Precedence::Inert)),
                "enum" => Some((TokenVariant::Keyword(Keyword::Enum), Precedence::Inert)),
                "rename" => Some((TokenVariant::Keyword(Keyword::Rename), Precedence::Inert)),
                "props" => Some((TokenVariant::Keyword(Keyword::Props), Precedence::Inert)),
                "tmpl" => Some((TokenVariant::Keyword(Keyword::Template), Precedence::Inert)),
                "pattern" => Some((TokenVariant::Keyword(Keyword::Pattern), Precedence::Inert)),
                "main" => Some((TokenVariant::Keyword(Keyword::Main), Precedence::Inert)),
                "let" => Some((TokenVariant::Keyword(Keyword::Let), Precedence::Inert)),
                "var" => Some((TokenVariant::Keyword(Keyword::Var), Precedence::Inert)),
                "if" => Some((TokenVariant::Keyword(Keyword::If), Precedence::Inert)),
                "elif" => Some((TokenVariant::Keyword(Keyword::Elif), Precedence::Inert)),
                "else" => Some((TokenVariant::Keyword(Keyword::Else), Precedence::Inert)),
                "for" => Some((TokenVariant::Keyword(Keyword::For), Precedence::Inert)),
                "ext" => Some((TokenVariant::Keyword(Keyword::Ext), Precedence::Inert)),
                "while" => Some((TokenVariant::Keyword(Keyword::While), Precedence::Inert)),
                "do" => Some((TokenVariant::Keyword(Keyword::Do), Precedence::Inert)),
                "switch" => Some((TokenVariant::Keyword(Keyword::Switch), Precedence::Inert)),
                "case" => Some((TokenVariant::Keyword(Keyword::Case), Precedence::Inert)),
                "_" => Some((TokenVariant::Keyword(Keyword::Default), Precedence::Inert)),
                "return" => Some((TokenVariant::Keyword(Keyword::Return), Precedence::Inert)),
                "break" => Some((TokenVariant::Keyword(Keyword::Break), Precedence::Inert)),
                // operators TBA
                "and" => Some((TokenVariant::Binary(Binary::And), Precedence::And)),
                _ => None,
            };
        }
        let value = self.token_value();

        match get_symbol(value) {
            None => {
                let id = self.sess.get_symbol_id(value);
                self.add_token(TokenVariant::Atom(Atom::Identifier(id)))
            }
            Some((cls, _precedence)) => self.add_token(cls),
        }
    }
    fn lex_special(&mut self) -> Result<(), ParserError> {
        let c = self.top();
        self.pass();
        match c {
            '=' => match self.top() {
                '=' => {
                    // ==
                    self.pass();
                    self.add_token(TokenVariant::Binary(Binary::Eq))
                }
                '>' => {
                    // =>
                    self.pass();
                    self.add_token(TokenVariant::Binary(Binary::Arrow))
                }
                _ => {
                    // =
                    self.add_token(TokenVariant::Binary(Binary::Be))
                }
            },
            ':' => {
                match self.top() {
                    ':' => {
                        self.pass();
                        self.add_token(TokenVariant::Join(Join::ModuleQualifier))
                    }
                    _ => todo!(), // throw_syntax_error(token_receiver.last().input,
                                  //                    SyntaxErrorClass::UnrecognizedToken,
                                  //                    "Donald Knuth and Sasha Rakhlin say don't "
                                  //                    "overdo the use of colons.\n"
                                  //                    "  So unlike python, husky doesn't put "
                                  //                    "colon at the end of block "
                                  //                    "head.");
                }
            }
            '(' => self.add_token(TokenVariant::Bra(Bracket::Par)),
            '[' => self.add_token(TokenVariant::Bra(Bracket::Box)),
            '{' => self.add_token(TokenVariant::Bra(Bracket::Curl)),
            ')' => self.add_token(TokenVariant::Ket(Bracket::Par)),
            ']' => self.add_token(TokenVariant::Ket(Bracket::Box)),
            '}' => self.add_token(TokenVariant::Ket(Bracket::Curl)),
            ',' => self.add_token(TokenVariant::Join(Join::Comma)),
            '&' => {
                match self.top() {
                    '&' => {
                        // &&
                        self.pass();
                        self.add_token(TokenVariant::Binary(Binary::And))
                    }
                    ' ' => {
                        // & space after
                        if !(!self.tokens.empty() && !self.tokens.last().is_rear_attached) {
                            todo!(); // syntax_assert(not token_receiver.empty() and
                                     //                  !token_receiver.last().is_rear_attached,
                                     //               token_receiver.last().input,
                                     //               SyntaxErrorClass::WrongSpacing,
                                     //               "& space after");
                        }
                        self.add_token(TokenVariant::Binary(Binary::BitAnd))
                    }
                    _ =>
                    // & no space after
                    {
                        self.add_token(TokenVariant::Prefix(Prefix::Shared))
                    }
                }
            }
            '|' => {
                match self.top() {
                    '|' => {
                        // ||
                        self.pass();
                        return self.add_token(TokenVariant::Binary(Binary::Or));
                    }
                    _ =>
                    // |
                    {
                        self.add_token(TokenVariant::Binary(Binary::BitOr))
                    }
                }
            }
            '~' => {
                return self.add_token(TokenVariant::Prefix(Prefix::BitNot));
            }
            '.' => {
                if self.top() == ' ' {
                    todo!() // syntax_assert(!token_receiver.empty() &&
                            //                   token_receiver.last().is_rear_attached,
                            //               token_receiver.last().input,
                            //               SyntaxErrorClass::WrongSpacing,
                            //               "func composition operator");
                            // err("TODO: func composition operator");
                } else {
                    if !self.tokens.empty() && !self.tokens.last().is_rear_attached {
                        syntax_error!(
                            self.tokens.last().range,
                            "pub def should be head".to_string(),
                            WrongKeyword
                        )
                    // self.tokens.last().input,
                    // SyntaxErrorClass::WrongSpacing,
                    // "member access"}else{Ok(())}?;
                    } else {
                        self.add_token(TokenVariant::Binary(Binary::MemberAccess))
                    }
                }
            }
            '%' => self.add_token(TokenVariant::Binary(Binary::Modulo)),
            '-' => {
                if self.top() == '=' {
                    // -=
                    self.pass();
                    self.add_token(TokenVariant::Binary(Binary::SubAssign))
                } else if self.top() == '-' {
                    // --
                    self.pass();
                    return self.add_token(TokenVariant::Suffix(Suffix::Decr));
                } else {
                    if self.tokens.right_convexity() == Convexity::Concave
                        || (self.tokens.empty() || !self.tokens.last().is_rear_attached) && c != ' '
                    {
                        self.add_token(TokenVariant::Prefix(Prefix::Minus))
                    } else {
                        self.add_token(TokenVariant::Binary(Binary::Sub))
                    }
                }
            }
            '<' => {
                match self.top() {
                    '<' => {
                        // <<
                        self.pass();
                        self.add_token(TokenVariant::Binary(Binary::RShift))
                    }
                    '=' => {
                        // <=
                        self.pass();
                        self.add_token(TokenVariant::Binary(Binary::Leq))
                    }
                    _ => self.add_token(TokenVariant::Binary(Binary::Less)),
                }
            }
            '>' => {
                match self.top() {
                    '>' => {
                        // >>
                        self.pass();
                        self.add_token(TokenVariant::Binary(Binary::LShift))
                    }
                    '=' => {
                        // >=
                        self.pass();
                        self.add_token(TokenVariant::Binary(Binary::Geq))
                    }
                    _ => self.add_token(TokenVariant::Binary(Binary::Greater)),
                }
            }
            '*' => {
                match self.top() {
                    '*' => {
                        // <<
                        self.pass();
                        todo!()
                    }
                    '=' => {
                        // <<
                        self.pass();
                        todo!()
                    }
                    _ => self.add_token(TokenVariant::Binary(Binary::Mult)),
                }
            }
            '/' => {
                match self.top() {
                    '/' => {
                        // // comments
                        self.stream.pass_to_eol();
                        self.add_token(TokenVariant::Keyword(Keyword::Comment))
                        // return self.add_token(TokenVariant::Keyword(Keyword::Comment));
                    }
                    '=' => {
                        // <<
                        self.pass();
                        todo!()
                    }
                    _ => self.add_token(TokenVariant::Binary(Binary::Div)),
                }
            }
            '+' => {
                match self.top() {
                    '+' => {
                        // ++
                        self.pass();
                        self.add_token(TokenVariant::Suffix(Suffix::Incr))
                    }
                    '=' => {
                        // +=
                        self.pass();
                        todo!()
                    }
                    _ => return self.add_token(TokenVariant::Binary(Binary::Add)),
                }
            }
            '!' => {
                match self.top() {
                    '=' => {
                        // !=
                        self.pass();
                        self.add_token(TokenVariant::Binary(Binary::Neq))
                    }
                    _ => self.add_token(TokenVariant::Prefix(Prefix::NotOrExclusive)),
                }
            }
            _ => todo!(),
        }
    }
    fn add_token(&mut self, cls: TokenVariant) -> Result<(), ParserError> {
        self.tokens
            .add_token(self.get_range(), cls, self.stream.top() != ' ')
    }
    // fn print_info(&self) {
    //   todo!();
    // }
}
