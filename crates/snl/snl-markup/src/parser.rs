use crate::*;
use husky_text_protocol::{char::TextCharIter, offset::TextOffset};

pub struct SnlMarkupParser<'a> {
    db: &'a EternerDb,
    markup_content: &'a str,
    char_iter: TextCharIter<'a>,
}

impl<'a> SnlMarkupParser<'a> {
    pub(crate) fn new(db: &'a EternerDb, markup_content: &'a str) -> Self {
        Self {
            db,
            markup_content,
            char_iter: TextCharIter::new(markup_content),
        }
    }
}

impl<'a> SnlMarkupParser<'a> {
    pub(crate) fn parse_pattern_command(&mut self) -> Option<TextOffsetRange> {
        self.char_iter
            .find_pattern(PATTERN_COMMAND)
            .map(|(offset_range, _)| offset_range)
    }

    pub(crate) fn parse_curly_braced<R>(
        &mut self,
        f: impl FnOnce(&mut Self) -> R,
    ) -> Option<(TextOffset, R, Option<TextOffset>)> {
        self.char_iter.eat_chars_while(|c| c == ' ');
        match self.char_iter.peek() {
            Some('{') => {
                let lcurl_offset = self.char_iter.current_offset();
                self.char_iter.eat_char();
                let result = f(self);
                self.char_iter.eat_chars_while(|c| c == ' ');
                match self.char_iter.peek() {
                    Some('}') => {
                        let rcurl_offset = self.char_iter.current_offset();
                        self.char_iter.eat_char();
                        Some((lcurl_offset, result, Some(rcurl_offset)))
                    }
                    _ => Some((lcurl_offset, result, None)),
                }
            }
            _ => None,
        }
    }

    pub(crate) fn parse_pattern_arguments(&mut self) -> SnlMarkupResult<SnlMarkupPatternArguments> {
        let mut layer = 0;
        let mut pattern_arguments = smallvec![];
        while let Some(pattern_argument) = self.parse_pattern_argument(&mut layer)? {
            pattern_arguments.push(pattern_argument);
        }
        Ok(pattern_arguments)
    }

    fn parse_pattern_argument(
        &mut self,
        layer: &mut usize,
    ) -> SnlMarkupResult<Option<SnlMarkupPatternArgument>> {
        let Some(command_offset_range) = self.parse_pattern_argument_command(layer) else {
            return Ok(None);
        };
        let Some((key_lcurl_offset, key_ident, key_rcurl_offset)) =
            self.parse_curly_braced(|parser| parser.parse_ident())
        else {
            return todo!("err");
        };
        let Some(key_rcurl_offset) = key_rcurl_offset else {
            return todo!("err");
        };
        let (key_offset_range, key_ident) = key_ident?;
        let Some((content_lcurl_offset, (), content_rcurl_offset)) =
            self.parse_curly_braced(|parser| parser.eat_until_outstanding_rcurl())
        else {
            return todo!("err");
        };
        let Some(content_rcurl_offset) = content_rcurl_offset else {
            return todo!("err");
        };
        Ok(Some(SnlMarkupPatternArgument {
            command_offset_range,
            key_curled_offset_range: TextOffsetRange::new(key_lcurl_offset, key_rcurl_offset),
            key_ident,
            value_curled_offset_range: TextOffsetRange::new(
                content_lcurl_offset,
                content_rcurl_offset,
            ),
            value_content: self.markup_content
                [(content_lcurl_offset.index() + 1)..(content_rcurl_offset.index())]
                .to_string(),
        }))
    }

    fn parse_pattern_argument_command(&mut self, layer: &mut usize) -> Option<TextOffsetRange> {
        loop {
            let offset_start = self.char_iter.current_offset();
            match self.char_iter.peek() {
                Some('\\') => {
                    self.char_iter.eat_char();
                    let ident = self
                        .char_iter
                        .next_str_slice_while(|c| c.is_ascii_alphabetic());
                    if ident.is_empty() {
                        self.char_iter.eat_char();
                    } else if ident == "patternArgument" {
                        let offset_end = self.char_iter.current_offset();
                        return Some(TextOffsetRange::new(offset_start, offset_end));
                    }
                }
                Some('{') => {
                    *layer += 1;
                    self.char_iter.eat_char();
                }
                Some('}') => {
                    if *layer == 0 {
                        break None;
                    }
                    *layer -= 1;
                    self.char_iter.eat_char();
                }
                _ => self.char_iter.eat_char(),
            }
        }
    }

    fn parse_ident(&mut self) -> SnlMarkupResult<(TextOffsetRange, SnlIdent)> {
        self.char_iter.eat_chars_while(|c| c == ' ');
        let offset_start = self.char_iter.current_offset();
        let slice = self
            .char_iter
            .next_str_slice_while(|c| c.is_ascii_alphanumeric() || c == '_');
        let offset_end = self.char_iter.current_offset();
        let ident = SnlIdent::from_ref(slice, self.db).map_err(|_| todo!())?;
        Ok((TextOffsetRange::new(offset_start, offset_end), ident))
    }

    fn eat_until_outstanding_rcurl(&mut self) {
        let mut layer = 0;
        loop {
            let offset_start = self.char_iter.current_offset();
            match self.char_iter.peek() {
                Some('\\') => {
                    self.char_iter.eat_char();
                    self.char_iter.eat_char();
                }
                Some('{') => {
                    layer += 1;
                    self.char_iter.eat_char();
                }
                Some('}') => {
                    if layer == 0 {
                        break;
                    }
                    layer -= 1;
                    self.char_iter.eat_char();
                }
                _ => self.char_iter.eat_char(),
            }
        }
    }
}

struct SnlMarkupPatternCommand {
    pattern_command_ident_and_lcurl_offset_range: TextOffsetRange,
}
