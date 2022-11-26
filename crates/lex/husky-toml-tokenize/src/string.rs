use crate::*;

impl<'a> Tokenizer<'a> {
    #[allow(clippy::type_complexity)]
    fn next_string(
        &mut self,
        delim: char,
        start: usize,
        new_ch: &mut dyn FnMut(
            &mut Tokenizer<'_>,
            &mut MaybeString,
            bool,
            usize,
            char,
        ) -> TomlTokenizeResult<()>,
    ) -> TomlTokenizeResult<TomlTokenVariant> {
        let mut multiline = false;
        if self.eatc(/* second */ delim) {
            if self.eatc(/* third */ delim) {
                multiline = true;
            } else {
                return Ok(TomlTokenVariant::StringLiteral {
                    val: Default::default(),
                    multiline: false,
                });
            }
        }
        let mut val = MaybeString::NotEscaped(self.current());
        let mut n = 0;
        'outer: loop {
            n += 1;
            match self.take_one_char() {
                Some((i, '\n')) => {
                    if multiline {
                        if self.input.as_bytes()[i] == b'\r' {
                            val.to_owned(&self.input[..i]);
                        }
                        if n == 1 {
                            val = MaybeString::NotEscaped(self.current());
                        } else {
                            val.push('\n');
                        }
                        continue;
                    } else {
                        return Err(TomlTokenizeError::NewlineInString(i));
                    }
                }
                Some((mut i, ch)) if ch == delim => {
                    if multiline {
                        if !self.eatc(delim) {
                            val.push(delim);
                            continue 'outer;
                        }
                        if !self.eatc(delim) {
                            val.push(delim);
                            val.push(delim);
                            continue 'outer;
                        }
                        if self.eatc(delim) {
                            val.push(delim);
                            i += 1;
                        }
                        if self.eatc(delim) {
                            val.push(delim);
                            i += 1;
                        }
                    }
                    return Ok(TomlTokenVariant::StringLiteral {
                        val: val.into_cow(&self.input[..i]),
                        multiline,
                    });
                }
                Some((i, c)) => new_ch(self, &mut val, multiline, i, c)?,
                None => return Err(TomlTokenizeError::UnterminatedString(start)),
            }
        }
    }

    pub(crate) fn next_literal_string(
        &mut self,
        start: usize,
    ) -> TomlTokenizeResult<TomlTokenVariant> {
        self.next_string('\'', start, &mut |_me, val, _multi, i, ch| {
            if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') {
                val.push(ch);
                Ok(())
            } else {
                Err(TomlTokenizeError::InvalidCharInString(i, ch))
            }
        })
    }

    pub(crate) fn next_basic_string(
        &mut self,
        start: usize,
    ) -> TomlTokenizeResult<TomlTokenVariant> {
        self.next_string('"', start, &mut |me, val, multi, i, ch| match ch {
            '\\' => {
                val.to_owned(&me.input[..i]);
                match me.chars.next() {
                    Some((_, '"')) => val.push('"'),
                    Some((_, '\\')) => val.push('\\'),
                    Some((_, 'b')) => val.push('\u{8}'),
                    Some((_, 'f')) => val.push('\u{c}'),
                    Some((_, 'n')) => val.push('\n'),
                    Some((_, 'r')) => val.push('\r'),
                    Some((_, 't')) => val.push('\t'),
                    Some((i, c @ 'u')) | Some((i, c @ 'U')) => {
                        let len = if c == 'u' { 4 } else { 8 };
                        val.push(me.next_hex(start, i, len)?);
                    }
                    Some((i, c @ ' ')) | Some((i, c @ '\t')) | Some((i, c @ '\n')) if multi => {
                        if c != '\n' {
                            while let Some((_, ch)) = me.chars.clone().next() {
                                match ch {
                                    ' ' | '\t' => {
                                        me.chars.next();
                                        continue;
                                    }
                                    '\n' => {
                                        me.chars.next();
                                        break;
                                    }
                                    _ => return Err(TomlTokenizeError::InvalidEscape(i, c)),
                                }
                            }
                        }
                        while let Some((_, ch)) = me.chars.clone().next() {
                            match ch {
                                ' ' | '\t' | '\n' => {
                                    me.chars.next();
                                }
                                _ => break,
                            }
                        }
                    }
                    Some((i, c)) => return Err(TomlTokenizeError::InvalidEscape(i, c)),
                    None => return Err(TomlTokenizeError::UnterminatedString(start)),
                }
                Ok(())
            }
            ch if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') => {
                val.push(ch);
                Ok(())
            }
            _ => Err(TomlTokenizeError::InvalidCharInString(i, ch)),
        })
    }
}
