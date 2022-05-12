use core::hash::Hash;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

#[derive(Copy, Clone)]
pub struct CustomIdentifier(pub &'static str);

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
