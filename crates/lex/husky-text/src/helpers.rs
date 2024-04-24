use crate::Text;
use husky_text_protocol::position::TextLine;

pub struct CommentLineInjection {
    start: TextLine,
    end: TextLine,
}

impl CommentLineInjection {
    pub fn inject_as_comment<'a>(
        text: Text<'a>,
        content: &str,
        end: TextLine,
        prefix: &str,
    ) -> CommentLineInjection {
        let mut start = end;
        while let Some(prev_line) = start - 1 {
            let text_within_prev_line = &text.text_within(prev_line);
            if text_within_prev_line.starts_with("//") {
                let trimmed = &text_within_prev_line[2..];
                if trimmed.starts_with(prefix) {
                    start = prev_line
                }
            }
        }
        CommentLineInjection { start, end }
    }
}

#[test]
fn comment_line_injection_works() {
    // todo!()
}
