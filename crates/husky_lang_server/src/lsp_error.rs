#[derive(Debug)]
pub(super) struct LspError {
    pub(super) code: i32,
    pub(super) message: String,
}

impl LspError {
    fn new(code: i32, message: String) -> LspError {
        LspError { code, message }
    }
}

impl std::fmt::Display for LspError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Language Server request failed with {}. ({})",
            self.code, self.message
        )
    }
}

impl std::error::Error for LspError {}
