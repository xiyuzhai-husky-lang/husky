use super::*;
use llm_prelude::transformation::LlmStringTransformationInstruction;

pub(super) fn simplification_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![AllLlmsStringTransformation {
        model: AllLlmModel::GEMINI_1_5_FLASH,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "Please provide the raw solution to the following problem".to_string(),
            side: format!(
                r#"
You should give directly the latex code for the solution, without any other text. Don't include \begin{{document}} or \end{{document}} or \begin{{proof}} or \end{{proof}}. Just the latex code inside the proof environment for the solution."#
            ),
        },
        examples: vec![],
    }]
}

pub(super) fn visored_preprocessing_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![AllLlmsStringTransformation {
        model: AllLlmModel::GEMINI_1_5_FLASH,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "Please provide a simplified solution to the following problem".to_string(),
            side: format!(
                r#"
You should give directly the latex code for the solution, without any other text. Don't include \begin{{document}} or \end{{document}} or \begin{{proof}} or \end{{proof}}. Just the latex code inside the proof environment for the solution. Don't include any \label or \ref."#
            ),
        },
        examples: vec![],
    }]
}
