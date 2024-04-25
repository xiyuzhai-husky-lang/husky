use crate::*;
use husky_text_protocol::{char::TextCharIter, position::TextLine};

pub struct CommentInjector<'a> {
    text: Text<'a>,
    prefix: &'a str,
    /// the ith string corresponds to the ith injections
    injections: Vec<String>,
}

/// # constructors

impl<'a> CommentInjector<'a> {
    pub fn new(text: Text<'a>, prefix: &'a str) -> Self {
        Self {
            text,
            prefix,
            injections: text.indexed_lines().map(|_| String::new()).collect(),
        }
    }
}

/// # actions

impl<'a> CommentInjector<'a> {
    pub fn inject(&mut self, text_line: impl Into<TextLine>, content: impl AsRef<str>) {
        let text_line = text_line.into();
        assert!(!self.skip_line(text_line));
        let injection = &mut self.injections[text_line.0 as usize];
        let indent_slice =
            TextCharIter::new(self.text.text_within(text_line)).next_str_slice_while(|c| c == ' ');
        let content = content.as_ref();
        for line in content.lines() {
            *injection += indent_slice;
            *injection += "//";
            *injection += self.prefix;
            *injection += line;
        }
    }

    fn skip_line(&self, text_line: TextLine) -> bool {
        let line = self.text.text_within(text_line);
        if line.starts_with("//") {
            if line[2..].starts_with(self.prefix) {
                return true;
            }
        }
        false
    }

    pub fn finish(self) -> String {
        let mut result = String::new();
        self.text.indexed_lines().zip(&self.injections).for_each(
            |((text_line, text_line_content), injection)| {
                if !self.skip_line(text_line) {
                    result += &injection;
                    if result.len() > 0 && !result.ends_with('\n') && text_line_content.len() > 0 {
                        result.push('\n')
                    }
                    result += text_line_content
                }
            },
        );
        result
    }
}

pub struct CommentLineInjection {
    start: TextLine,
    end: TextLine,
}

#[cfg(test)]
#[track_caller]
fn t(input: &str, injections: &[(u32, &str)], expected: &str) {
    run_test_on_text(
        input,
        #[track_caller]
        |text| {
            let mut injector = CommentInjector::new(text, "? ");
            for &(text_line, injection) in injections {
                injector.inject(text_line, injection)
            }
            let result = injector.finish();
            assert_eq!(&result, expected)
        },
    )
}

#[test]
fn comment_injector_works() {
    t("", &[], "");
    t("hello", &[], "hello");
    t(
        r#"hello
world"#,
        &[],
        r#"hello
world"#,
    );
    t(
        r#"hello
world"#,
        &[(0, "this is a verb")],
        r#"//? this is a verb
hello
world"#,
    );
    t(
        r#"//? this is a verb
hello
world"#,
        &[(1, "this is a verb")],
        r#"//? this is a verb
hello
world"#,
    );
}

#[test]
#[should_panic]
fn comment_injector_fails() {
    t(
        r#"//? this is a verb
hello
world"#,
        &[(0, "this is a verb")],
        r#"//? this is a verb
hello
world"#,
    );
}

#[test]
fn inject_multiple_lines() {
    t(
        r#"Line 1
Line 2
Line 3"#,
        &[(1, "first line\nsecond line")],
        r#"Line 1
//? first line
//? second line
Line 2
Line 3"#,
    );
}

#[test]
fn inject_with_special_characters() {
    t(
        r#"Special chars here:"#,
        &[(0, "chars: #@$%^&*()")],
        r#"//? chars: #@$%^&*()
Special chars here:"#,
    );
}

#[test]
fn inject_into_empty_file() {
    t("", &[(0, "insert into empty")], "//? insert into empty");
}

#[test]
fn inject_on_empty_line() {
    t(
        r#"

"#,
        &[(0, "comment on empty line")],
        r#"//? comment on empty line

"#,
    );
}

#[test]
fn consecutive_injections() {
    t(
        r#"start
middle
end"#,
        &[(0, "top comment"), (2, "bottom comment")],
        r#"//? top comment
start
middle
//? bottom comment
end"#,
    );
}

#[test]
fn inject_with_indentation_works() {
    t(
        r#"    indented line"#,
        &[(0, "with indentation")],
        r#"    //? with indentation
    indented line"#,
    );
}

#[test]
fn non_sequential_injections_works() {
    t(
        r#"One
Two
Three"#,
        &[(2, "last first"), (0, "first last")],
        r#"//? first last
One
Two
//? last first
Three"#,
    );
}
