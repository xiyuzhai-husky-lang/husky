use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DevSource {
    pub file: String,
    pub line: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct __StaticDevSource {
    pub file: &'static str,
    pub line: u32,
}

impl std::fmt::Display for __StaticDevSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.file, self.line))
    }
}

impl From<__StaticDevSource> for DevSource {
    fn from(__static_dev_src: __StaticDevSource) -> Self {
        Self {
            file: __static_dev_src.file.into(),
            line: __static_dev_src.line,
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
macro_rules! __static_dev_src {
    () => {
        __StaticDevSource {
            file: file!(),
            line: line!(),
        }
    };
}
