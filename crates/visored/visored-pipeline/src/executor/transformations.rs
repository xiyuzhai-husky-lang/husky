use super::*;
use llm_prelude::transformation::LlmStringTransformationInstruction;

pub(super) fn simplification_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_PRO,
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
            antiexamples: vec![],
        },
        AllLlmsStringTransformation {
            model: AllLlmModel::GEMINI_1_5_FLASH,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main:
                    "Please remove all labels in align environments from the following latex code:"
                        .to_string(),
                side: None,
            },
            examples: vec![r#"
---- EXAMPLE INPUT ----
```latex
...
We have $(a + f(x))^2$ being positive because these are real numbers. Therefore $(a + f(x))^2 \ge 0$.
...
```

---- EXAMPLE OUTPUT ----
```latex
\begin{{proof}}
...
We have $(a + f(x))^2  \ge 0$.
...
\end{{proof}}
```
"#
            .to_string()],
            antiexamples: vec![],
        },
    ]
}

pub(super) fn elaboration_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![AllLlmsStringTransformation {
        model: AllLlmModel::GEMINI_1_5_PRO,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "Elaborate formula derivation that skips too many steps by inserting steps in the given proof. You should only insert steps if it's absolutely necessary. Don't bother to explain how things are derived. For example, don't add extra since clauses because we don't want that. Definitely avoid expanding too much that breaks the form. If a step is already obvious, keep it intact. Keep the same number of sentences but put in more chains of equalities or inequalities. There should be one-to-one correspondence between sentences in the input and output."
                .to_string(),
            side: Some(
                r#"Wrap the proof in \begin{{proof}} and \end{{proof}}."#
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
        % these expansions make it more obvious how the well known fact $x^2+y^2 \ge 2xy$ is applied
        We have $a + b = (\sqrt{a})^2 + (\sqrt{b})^2 \ge 2 \sqrt{a} \sqrt{b} = 2\sqrt{ab}$.
        \end{{proof}}
        ```
        "#.to_string(),
        r#"
        ---- EXAMPLE INPUT ----
        ```latex
        Problem: ...

        \begin{{proof}}
        ...
        We have $(x+2)^2 \ge 0$ because these are real numbers.
        ...
        \end{{proof}}
        ```
        ---- EXAMPLE OUTPUT ----
        ```latex
        Problem: ...

        \begin{{proof}}
        ...
        We have $(x+2)^2 \ge 0$ because these are real numbers.
        ...
        \end{{proof}}
        ```

        This is because the original proof is already detailed enough.
        "#.to_string(),
        ],
        antiexamples: vec![
            r#"
The following antiexample is bad because it expands too much and actually make the proof wrong. The original proof is already quite obvious following by the well known fact $x^2$ for any real number $x$.

---- ANTI-EXAMPLE INPUT ----
```latex
We have $(x + y)^2 \ge 0$ because these are real numbers.
```

---- ANTI-EXAMPLE OUTPUT ----
```latex
\begin{{proof}}
...
We have $(x + y)^2 = (x + y)(x + y) = x^2 + 2xy + y^2 \ge 0$ because these are real numbers.
...
\end{{proof}}
```
            "#.to_string(),
            r#"
The following antiexample is bad because it expands too much and actually make the proof wrong. The original proof is already quite obvious following by the well known fact $x^2$ for any real number $x$.

---- ANTI-EXAMPLE INPUT ----
```latex
We have $(a + f(x))^2 \ge 0$ because these are real numbers.
```

---- ANTI-EXAMPLE OUTPUT ----
```latex
\begin{{proof}}
...
We have $(a + f(x))^2 = a^2 + 2af(x) + f(x)^2 \ge 0$ because these are real numbers.
...
\end{{proof}}
```
            "#.to_string(),
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
            antiexamples: vec![],
        },
]
}

pub(super) fn regularization_transformations() -> Vec<AllLlmsStringTransformation> {
    vec![AllLlmsStringTransformation {
        model: AllLlmModel::GEMINI_1_5_PRO,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "Regularize the proof by using only the following sentences:
- We have <proposition>. This is for stating a proposition derived in an obvious manner from existing propositions in forward reasoning.
- We have <proposition> by <reason>/ where <reasons>. This is for stating a nontrivial proposition derived from existing propositions in forward reasoning that requires some explanation.
- It's enough to show that <proposition>. This is for changing the goal of the proof in backward reasoning.
            "
                .to_string(),
            side: Some(
                r#"Wrap the proof in \begin{{proof}} and \end{{proof}}. Don't break the chain of equalities or inequalities. Keep chain of equalities or inequalities in one sentence. "#
                .to_string(),
            ),
        },
        examples: vec![],
        antiexamples: vec![],
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
            antiexamples: vec![],
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
        antiexamples: vec![],
    }]
}
