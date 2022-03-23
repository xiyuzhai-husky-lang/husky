#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DevSource {
    pub file: &'static str,
    pub line: u32,
}

impl std::fmt::Debug for DevSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", &self.file, &self.line))
    }
}

#[macro_export]
macro_rules! src {
    () => {
        dev_utils::DevSource {
            file: file!(),
            line: line!(),
        }
    };
}
