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
                side: "Addtionally, you should wrap the output in a ```text``` block.".to_string(),
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
        },
        expect![[r#"
            Replace each word `hello` with `goodbye` in the following text:

            ```text
            Hello, world!
            ```

            Addtionally, you should wrap the output in a ```text``` block.

            EXAMPLES:
            ---------
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
