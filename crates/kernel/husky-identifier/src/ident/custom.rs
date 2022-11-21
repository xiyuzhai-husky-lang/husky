use super::*;
use core::hash::Hash;
use interner::InternedRefWrapper;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub(crate) InternedRefWrapper<str>);
impl From<Identifier> for Identifier {
    fn from(ident: Identifier) -> Self {
        Self::Custom(ident)
    }
}

impl HuskyDisplay for Identifier {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <10}{}",
                husky_print_utils::CYAN,
                self.as_str(),
                husky_print_utils::RESET
            )
            .unwrap();
        } else {
            write!(result, "{: <10}", self.as_str()).unwrap();
        }
    }
}

impl Identifier {
    pub fn snake_name(&self) -> String {
        let mut snake_name = String::new();
        for c in self.0.chars() {
            if c.is_alphanumeric() {
                snake_name.push(c)
            } else {
                todo!()
            }
        }
        snake_name
    }
    pub fn dash_name(&self) -> String {
        todo!()
    }

    pub fn as_str(&self) -> &'static str {
        self.0.deref_static()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.0.deref_static())
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref_static()
    }
}

impl Borrow<str> for Identifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
