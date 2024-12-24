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
    /// You're doing some edits on user provided inputs. You will be given instructions and input. You should only return the edited input. Don't include any other text in any case.
    ///
    /// ----- MAIN INSTRUCTIONS -----
    /// {main}
    ///
    /// ----- INPUT -----
    /// {input}
    ///
    /// ----- SIDE INSTRUCTIONS -----
    /// {side}
    /// ```
    MainInputSide { main: String, side: Option<String> },
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
                    r#"You're doing some edits on user provided inputs. You will be given instructions and input. You should only return the edited input. Don't include any other text in any case.

----- MAIN INSTRUCTIONS -----
{main}

----- INPUT -----
{input}{}
"#,
                    match side.as_ref() {
                        Some(side) => format!(
                            r#"

----- SIDE INSTRUCTIONS -----
{side}"#
                        ),
                        None => "".to_string(),
                    }
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
