use super::*;
use expect_test::{expect, Expect};

#[test]
fn llm_string_transformation_prompt_works() {
    fn t(tfn: LlmStringTransformation<()>, expect: Expect) {
        let input = r#"```text
Hello, world!
```"#;
        let prompt = tfn.prompt(input);
        expect.assert_eq(&prompt);
    }

    t(
        LlmStringTransformation {
            model: (),
            instruction: LlmStringTransformationInstruction::MainInputSide {
                main: "Replace each word `hello` with `goodbye` in the following text:".to_string(),
                side: Some(
                    "Addtionally, you should wrap the output in a ```text``` block.".to_string(),
                ),
            },
            examples: vec![r#"Given:

```text
Hello, world!
```

The answer should be:

```text
Goodbye, world!
```"#
                .to_string()],
            antiexamples: vec![],
        },
        expect![[r#"
            You're doing some edits on user provided inputs. You will be given instructions and input. You should only return the edited input. Don't include any other text in any case.

            ----- MAIN INSTRUCTIONS -----
            Replace each word `hello` with `goodbye` in the following text:

            ----- INPUT -----
            ```text
            Hello, world!
            ```

            ----- SIDE INSTRUCTIONS -----
            Addtionally, you should wrap the output in a ```text``` block.


            Here are some examples that help you understand the task.

            ------- EXAMPLES -------
            Example 1:
            Given:

            ```text
            Hello, world!
            ```

            The answer should be:

            ```text
            Goodbye, world!
            ```
            ---------
        "#]],
    );
}
