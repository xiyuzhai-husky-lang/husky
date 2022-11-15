use super::*;

impl ExpectInstance {
    pub(crate) fn update(
        &mut self,
        f: &(impl Fn(&str) -> String + UnwindSafe + RefUnwindSafe),
    ) -> UpdateExpectResult<()> {
        let result = self.update_inner(f);
        self.save();
        result
    }

    fn update_inner(
        &mut self,
        f: &(impl Fn(&str) -> String + UnwindSafe + RefUnwindSafe),
    ) -> UpdateExpectResult<()> {
        for expect in self.entries() {
            let output = match catch_unwind_with_message(|| f(&expect.input)) {
                Ok(output) => output,
                Err(e) => return Err(UpdateExpectError::DerivedPanic(e)),
            };
            if expect.output.as_ref() != Some(&output) {
                match ask_is_input_output_okay(&expect.input, &output) {
                    true => expect.output = Some(output),
                    false => return Err(UpdateExpectError::OutputNotAccepted),
                }
            }
        }
        Ok(())
    }

    fn save(&self) {
        self.save_test_results_json();
        self.save_test_results_markdown()
    }

    fn save_test_results_json(&self) {
        diff_write(
            &self.test_results_json_path(),
            &serde_json::to_string_pretty(&self.entries).unwrap(),
            true,
        );
    }

    fn save_test_results_markdown(&self) {
        diff_write(
            &self.test_results_markdown_path(),
            &self.to_markdown(),
            true,
        );
    }

    fn to_markdown(&self) -> String {
        use std::fmt::Write;

        let file_stem = self.test_path_stem.file_name().unwrap().to_str().unwrap();
        let title = file_stem.to_case(Case::Title);
        let mut result: String = format!("# {title}",);
        for (i, entry) in self.entries.iter().enumerate() {
            write!(
                result,
                r#"

## Test#{i}

input

```husky
{}
```"#,
                entry.input
            );
            if let Some(ref output) = entry.output {
                write!(
                    result,
                    r#"

output

```husky
{}
```"#,
                    output
                );
            }
        }
        result.push('\n');
        result
    }
}
