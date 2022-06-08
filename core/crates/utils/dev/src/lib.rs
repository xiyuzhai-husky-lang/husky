use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DevSource {
    pub file: String,
    pub line: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct StaticDevSource {
    pub file: &'static str,
    pub line: u32,
}

impl From<StaticDevSource> for DevSource {
    fn from(static_dev_src: StaticDevSource) -> Self {
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
        dev_utils::DevSource {
            file: file!().to_string(),
            line: line!(),
        }
    };
}

#[macro_export]
macro_rules! static_dev_src {
    () => {
        dev_utils::StaticDevSource {
            file: file!(),
            line: line!(),
        }
    };
}
