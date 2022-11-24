use std::ops::Deref;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConfigKeyword {
    Task,
}

impl const Into<Keyword> for ConfigKeyword {
    fn into(self) -> Keyword {
        Keyword::Config(self)
    }
}

impl ConfigKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigKeyword::Task => "task",
        }
    }
}

impl Deref for ConfigKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
