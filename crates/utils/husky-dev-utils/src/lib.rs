use std::path::PathBuf;

#[derive(PartialEq, Eq, Clone)]
pub struct DevSource {
    pub file: String,
    pub line: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(C)]
pub struct __StaticDevSource {
    pub file: __StaticFile,
    pub line: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(C)]
pub enum __StaticFile {
    Rust(&'static str),
}

impl std::fmt::Display for __StaticFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            __StaticFile::Rust(file) => f.write_str(file),
        }
    }
}

impl From<__StaticFile> for PathBuf {
    fn from(val: __StaticFile) -> Self {
        match val {
            __StaticFile::Rust(file) => file.into(),
        }
    }
}

impl From<__StaticFile> for String {
    fn from(val: __StaticFile) -> Self {
        match val {
            __StaticFile::Rust(file) => file.to_string(),
        }
    }
}

impl std::fmt::Display for __StaticDevSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.file, self.line))
    }
}

impl From<__StaticDevSource> for DevSource {
    fn from(static_dev_src: __StaticDevSource) -> Self {
        Self {
            file: static_dev_src.file.into(),
            line: static_dev_src.line,
        }
    }
}

impl std::fmt::Debug for DevSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", &self.file, &self.line))
    }
}

#[macro_export]
macro_rules! dev_src {
    () => {
        husky_dev_utils::DevSource {
            file: file!().to_string(),
            line: line!(),
        }
    };
}

#[macro_export]
macro_rules! static_dev_src {
    () => {
        __StaticDevSource {
            file: __StaticFile::Rust(file!()),
            line: line!(),
        }
    };
}
