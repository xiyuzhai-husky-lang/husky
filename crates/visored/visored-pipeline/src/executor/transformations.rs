use super::*;
use llm_prelude::transformation::LlmStringTransformationInstruction;

impl VdPipelineConfig {
    pub(super) fn simplification_transformations(&self) -> Vec<AllLlmsStringTransformation> {
        vec![
        AllLlmsStringTransformation {
            model: self.routing_resolved.solver.mathematical_understanding.model,
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
            model: self.routing_resolved.solver.latex_rewriter.model,
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

    pub(super) fn elaboration_transformations(&self) -> Vec<AllLlmsStringTransformation> {
        vec![AllLlmsStringTransformation {
        model: self.routing_resolved.solver.mathematical_understanding.model,
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
            model: self.routing_resolved.solver.latex_rewriter.model,
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

    pub(super) fn regularization_transformations(&self) -> Vec<AllLlmsStringTransformation> {
        vec![AllLlmsStringTransformation {
        model: self.routing_resolved.solver.mathematical_understanding.model,
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
        model: self.routing_resolved.solver.mathematical_understanding.model,
        instruction: LlmStringTransformationInstruction::MainInputSide {
            main: "You're inserting sentences to the original proof. Don't modify the original proof other than that. In the beginning of the proof, introducing all variables and assumptions using the following format:
- Let <phrase>. This is for introducing a free variable or an assigned variable. Try to use formula as much as possible, such as `Let $x\\in\\mathbb{R}$` instead of `Let $x$ be a real number`. Try to isolate things as much as possible, i.e., declare one variable per sentence. For example, `Let $x\\in\\mathbb{R}$. Let $y\\in\\mathbb{R}$` is better than `Let $x,y\\in\\mathbb{R}$`.
- Assume <assumption>. This is for introducing an assumption. Try to use formula as much as possible, such as `Let $x\\in\\mathbb{R}$` instead of `Let $x$ be a real number`.
            "
                .to_string(),
            side: Some(
                r#"Sometimes you need to combine the two. For example, use `Let $x\\in\\mathbb{R}$. Assume $x\\ge 0$` to express `Let $x$ be a positive real number.`.
                Wrap the proof in \begin{{proof}} and \end{{proof}}. "#
                .to_string(),
            ),
        },
        examples: vec![],
        antiexamples: vec![],
    },
    AllLlmsStringTransformation {
            model: self.routing_resolved.solver.latex_rewriter.model,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main:
                    "For any superscript and subscript, if the intended base is not atomic latex expression, wrap it in curly braces. For example, $(a+b)^2$ should be ${{(a+b)}}^2$."
                        .to_string(),
                side: None,
            },
            examples: vec![],
            antiexamples: vec![],
        },
        AllLlmsStringTransformation {
            model: self.routing_resolved.solver.latex_rewriter.model,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main:
                    "Please remove all labels and refs in math environments. Use `$...$` for all math expressions."
                        .to_string(),
                side: None,
            },
            examples: vec![],
            antiexamples: vec![],
        },
]
    }

    pub(super) fn visored_preprocessing_transformations(&self) -> Vec<AllLlmsStringTransformation> {
        vec![AllLlmsStringTransformation {
            model: self.routing_resolved.solver.latex_rewriter.model,
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main: "Remove discussion of equality holds. These are not needed.".to_string(),
                side: None,
            },
            examples: vec![],
            antiexamples: vec![],
        }]
    }
}
