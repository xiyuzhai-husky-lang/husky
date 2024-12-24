use super::*;
use llm_prelude::transformation::LlmStringTransformationInstruction;

pub(super) fn simplification_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_FLASH,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main: "Please simplify the following mathematical proof:".to_string(),
                side: Some(format!(
                    r#"
    
    There should only be one solution. No need to include alternatives. If the problem is entirely trivial, just need to say that it's trivial that XXX holds.

    Wrap the proof in \begin{{proof}} and \end{{proof}}.
    "#
                )),
            },
            examples: vec![],
        },
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_FLASH,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main:
                    "Please remove all labels in align environments from the following latex code:"
                        .to_string(),
                side: None,
            },
            examples: vec![],
        },
    ]
}

pub(super) fn elaboration_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![AllLlmsStringTransformation {
        model: AllLlmModel::GEMINI_1_5_FLASH,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "Elaborate formula derivation in the given proof. Put in more intermediate formulas to make the proof more obvious and easier to check. Keep the same number of sentences but put in more chains of equalities or inequalities. There should be one-to-one correspondence between sentences in the input and output."
                .to_string(),
            side: Some(
                r#"

    Wrap the proof in \begin{{proof}} and \end{{proof}}. It is intended to be latex code contained in the document body, not a full document. Make sure to make it valid under latex text mode.
    "#
                .to_string(),
            ),
        },
        examples: vec![
            r#"
            ---- EXAMPLE INPUT ----
        ```latex
        We have $a + b \ge 2\sqrt{ab}$.
        ```

        ---- OUTPUT ----
        ```latex
        \begin{{proof}}
        We have $a + b = (\sqrt{a})^2 + (\sqrt{b})^2 \ge 2 \sqrt{a} \sqrt{b} = 2\sqrt{ab}$.
        \end{{proof}}
        ```
        "#.to_string()
        ],
    },
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_FLASH,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main:
                    "Please remove all labels in align environments from the following latex code:"
                        .to_string(),
                side: None,
            },
            examples: vec![],
        },
]
}

pub(super) fn visored_preprocessing_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![AllLlmsStringTransformation {
        model: AllLlmModel::GEMINI_1_5_FLASH,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "Remove discussion of equality holds. These are not needed.".to_string(),
            side: None,
        },
        examples: vec![],
    }]
}
