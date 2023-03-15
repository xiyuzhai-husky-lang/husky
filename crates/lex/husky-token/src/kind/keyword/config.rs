use std::ops::Deref;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConfigKeyword {
    Task,
}

impl ConfigKeyword {
    pub const fn code(&self) -> &'static str {
        match self {
            ConfigKeyword::Task => "task",
        }
    }
}

impl Deref for ConfigKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}
