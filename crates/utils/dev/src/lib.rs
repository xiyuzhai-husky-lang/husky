use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DevSource {
    pub file: String,
    pub line: u32,
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
