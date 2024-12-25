use crate::error::GeminiError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeminiTier {
    Free,
    Paid,
}

impl TryFrom<&str> for GeminiTier {
    type Error = GeminiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "FREE" | "free" => Ok(GeminiTier::Free),
            "PAID" | "paid" => Ok(GeminiTier::Paid),
            _ => Err(GeminiError::InvalidTier(value.to_owned())),
        }
    }
}
