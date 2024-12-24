#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LlmStringTransformation<Model> {
    pub model: Model,
    pub instruction: LlmStringTransformationInstruction,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LlmStringTransformationInstruction {
    /// The prompt will be like:
    /// ```text
    /// {main}
    ///
    /// {input}
    ///
    /// {side}
    /// ```
    MainInputSide { main: String, side: String },
}

impl<Model> LlmStringTransformation<Model> {
    pub fn prompt(&self, input: &str) -> String {
        let mut prompt = self.instruction.prompt(input);
        if self.examples.len() > 0 {
            prompt += "\n\nEXAMPLES:\n---------\n";
            for (i, example) in self.examples.iter().enumerate() {
                prompt += &format!("Example {}:\n", i + 1);
                prompt += example;
                prompt += "\n---------\n";
            }
        }
        prompt
    }
}

impl LlmStringTransformationInstruction {
    pub fn prompt(&self, input: &str) -> String {
        match self {
            LlmStringTransformationInstruction::MainInputSide { main, side } => {
                format!(
                    r#"{main}

{input}

{side}"#
                )
            }
        }
    }
}

pub struct LlmStringTransformationRecord<Model> {
    pub transformation: LlmStringTransformation<Model>,
    pub input: String,
    pub prompt: String,
    pub output: String,
}
