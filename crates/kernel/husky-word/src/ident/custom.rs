use super::*;
use core::hash::Hash;
use interner::InternedRefWrapper;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CustomIdentifier(pub(crate) InternedRefWrapper<str>);
impl From<CustomIdentifier> for Identifier {
    fn from(ident: CustomIdentifier) -> Self {
        Self::Custom(ident)
    }
}

impl HuskyDisplay for CustomIdentifier {
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

impl CustomIdentifier {
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

impl Display for CustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.0.deref_static())
    }
}

impl Deref for CustomIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref_static()
    }
}

impl Borrow<str> for CustomIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
