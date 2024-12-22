use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct Part {
    pub text: String,
}
