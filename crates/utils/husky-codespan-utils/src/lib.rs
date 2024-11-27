use codespan_reporting::{
    files::{Files, SimpleFiles},
    term::{emit, Config},
};

pub fn emit_to_stdout(content: &str, offset_range: std::ops::Range<usize>, message: String) {
    let mut files = SimpleFiles::new();
    let file_id = files.add("input", content);

    let diagnostic = codespan_reporting::diagnostic::Diagnostic::error()
        .with_message(message)
        .with_labels(vec![codespan_reporting::diagnostic::Label::primary(
            file_id,
            offset_range,
        )]);

    let writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
    let config = Config::default();

    emit(&mut writer.lock(), &config, &files, &diagnostic).expect("Failed to emit diagnostic");
}
