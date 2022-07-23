use super::*;
use core::hash::Hash;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

#[derive(Copy, Clone)]
pub struct CustomIdentifier(pub &'static str);
impl From<CustomIdentifier> for Identifier {
    fn from(ident: CustomIdentifier) -> Self {
        Self::Custom(ident)
    }
}

impl TestDisplay for CustomIdentifier {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
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
        self.0
    }
}

impl std::fmt::Debug for CustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Display for CustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.0)
    }
}

impl PartialEq for CustomIdentifier {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as *const str) == (other.0 as *const str)
    }
}

impl Eq for CustomIdentifier {}

impl Hash for CustomIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const str).hash(state);
    }
}

impl Deref for CustomIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Borrow<str> for CustomIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
