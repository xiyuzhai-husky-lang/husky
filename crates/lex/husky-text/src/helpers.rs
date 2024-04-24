use crate::Text;
use husky_text_protocol::position::TextLine;

pub fn inject_as_comment<'a>(
    text: Text<'a>,
    content: &str,
    before_which_line: TextLine,
    prefix: &str,
) -> std::borrow::Cow<'a, str> {
    if !is_already_injected(text, content, before_which_line) {
        return text.raw_text.into();
    }
    todo!()
}

fn is_already_injected<'a>(text: Text<'a>, content: &str, before_which_line: TextLine) -> bool {
    todo!()
}

#[test]
fn inject_as_comment_works() {
    todo!()
}
