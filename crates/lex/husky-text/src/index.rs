use super::Text;
use husky_text_protocol::{
    position::{TextLine, TextPosition},
    range::TextRange,
};
use ref_index::RefIndex;

#[cfg(test)]
pub fn run_test_on_text(raw_text: &str, f: impl FnOnce(Text)) {
    use husky_text_protocol::line_map::LineMap;

    let line_map = &LineMap::new(raw_text);
    f(Text { raw_text, line_map });
}

impl<'a> Text<'a> {
    pub fn text_within(self, range: TextRange) -> &'a str {
        &self.raw_text[self.line_map.offset_range(range)]
    }
    pub fn text_within2<R>(self, range: R) -> &'a str
    where
        Self: RefIndex<'a, R, Output = str>,
    {
        self.ref_index(range)
    }
}

impl<'a> RefIndex<'a, std::ops::Range<(u32, u32)>> for Text<'a> {
    type Output = str;

    fn ref_index(self, range: std::ops::Range<(u32, u32)>) -> &'a Self::Output {
        &self.raw_text[self.line_map.offset_range(range)]
    }
}

impl<'a> std::ops::Index<std::ops::Range<(u32, u32)>> for Text<'a> {
    type Output = str;

    fn index(&self, range: std::ops::Range<(u32, u32)>) -> &Self::Output {
        &self.raw_text[self.line_map.offset_range(range)]
    }
}

impl<'a> std::ops::Index<TextRange> for Text<'a> {
    type Output = str;

    fn index(&self, range: TextRange) -> &Self::Output {
        &self.raw_text[self.line_map.offset_range(range)]
    }
}

impl<'a> std::ops::Index<std::ops::RangeFrom<TextPosition>> for Text<'a> {
    type Output = str;

    fn index(&self, range_from: std::ops::RangeFrom<TextPosition>) -> &Self::Output {
        &self.raw_text[self.line_map.offset_range_from(range_from)]
    }
}

#[cfg(test)]
pub(crate) fn run_index_test_on_text<I>(input: &str, range_from: I, expected: &str)
where
    for<'a> Text<'a>: std::ops::Index<I, Output = str>,
{
    run_test_on_text(input, |text| assert_eq!(&text[range_from], expected))
}

#[test]
pub(crate) fn range_from_text_position_works() {
    let t = run_index_test_on_text::<std::ops::RangeFrom<TextPosition>>;
    t("hello", ((0, 1).into()).., "ello");
}

#[test]
#[should_panic]
pub(crate) fn range_from_text_position_fails() {
    let t = run_index_test_on_text::<std::ops::RangeFrom<TextPosition>>;
    t("hello\n", ((1, 1).into()).., "ello");
}

impl<'a> std::ops::Index<std::ops::RangeTo<TextPosition>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::RangeTo<TextPosition>) -> &Self::Output {
        todo!()
    }
}

impl<'a> std::ops::Index<std::ops::RangeInclusive<TextPosition>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::RangeInclusive<TextPosition>) -> &Self::Output {
        todo!()
    }
}

impl<'a> std::ops::Index<std::ops::RangeToInclusive<TextPosition>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::RangeToInclusive<TextPosition>) -> &Self::Output {
        todo!()
    }
}

impl<'a> std::ops::Index<std::ops::RangeFrom<TextLine>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::RangeFrom<TextLine>) -> &Self::Output {
        todo!()
    }
}

impl<'a> std::ops::Index<std::ops::RangeTo<TextLine>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::RangeTo<TextLine>) -> &Self::Output {
        todo!()
    }
}

impl<'a> std::ops::Index<std::ops::RangeInclusive<TextLine>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::RangeInclusive<TextLine>) -> &Self::Output {
        todo!()
    }
}

impl<'a> std::ops::Index<std::ops::RangeToInclusive<TextLine>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::RangeToInclusive<TextLine>) -> &Self::Output {
        todo!()
    }
}
