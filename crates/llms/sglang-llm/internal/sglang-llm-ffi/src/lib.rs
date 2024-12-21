pub mod chat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    Qwen2,
    Qwen2Math,
}

impl TryFrom<&str> for Model {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Qwen2" | "qwen2" | "QWEN2" => Ok(Model::Qwen2),
            "Qwen2Math" | "qwen2-math" | "QWEN2-MATH" => Ok(Model::Qwen2Math),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_try_from() {
        assert_eq!(Model::try_from("Qwen2").unwrap(), Model::Qwen2);
    }
}
