mod transformations;

use crate::*;
use all_llms::{model::AllLlmModel, AllLlmsClient};
use eterned::db::EternerDb;
use input::VdPipelineInput;
use std::sync::Arc;

pub struct VdPipelineExecutor<'a, 'db> {
    input: &'a VdPipelineInput,
    config: &'a VdPipelineConfig,
    llm_client: AllLlmsClient<'db>,
    raw_solution: Option<String>,
    simplified_solution: Option<String>,
}

impl<'a, 'db> VdPipelineExecutor<'a, 'db> {
    pub fn new(
        db: &'db EternerDb,
        input: &'a VdPipelineInput,
        config: &'a VdPipelineConfig,
    ) -> Self {
        let base = input.file_path.parent().unwrap();
        let cache_dir = config.data.cache_dir.to_logical_path(base).join(format!(
            "{}/example-{}",
            input.file_path.file_stem().unwrap().to_str().unwrap(),
            input.index
        ));
        std::fs::create_dir_all(&cache_dir).unwrap();
        Self {
            input,
            config,
            llm_client: AllLlmsClient::new(db, cache_dir).unwrap(),
            raw_solution: None,
            simplified_solution: None,
        }
    }
}

impl<'a, 'db> VdPipelineExecutor<'a, 'db> {
    pub(crate) fn execute_all(&mut self) {
        self.query_raw_solution();
    }

    fn query_raw_solution(&mut self) {
        let prompt = format!(
            r#"Please provide the raw solution to the following problem:
```latex
{}
```
"#,
            self.input.content
        );
        // TODO: use config
        let model = AllLlmModel::GEMINI_1_5_FLASH;
        self.raw_solution = Some(self.llm_client.generate_text(model, prompt).unwrap());
        let prompt = format!(
            r#"Please provide a simplified solution to the following problem:
```latex
{}
```

The previous solution is:
```latex
{}
```

You should give directly the latex code for the solution, without any other text. Don't include \begin{{document}} or \end{{document}} or \begin{{proof}} or \end{{proof}}. Just the latex code inside the proof environment for the solution. Don't include any \label or \ref."#,
            self.input.content,
            self.raw_solution.as_ref().unwrap(),
        );
        self.simplified_solution = Some(extract_latex(
            &self.llm_client.generate_text(model, prompt).unwrap(),
        ));
    }

    pub(crate) fn finish(self) -> (String, String) {
        (
            self.raw_solution.unwrap(),
            self.simplified_solution.unwrap(),
        )
    }
}

fn extract_latex(s: &str) -> String {
    if let (Some(start), Some(end)) = (s.find("```latex"), s.rfind("```")) {
        let content = &s[start + 8..end];
        content.trim().to_string()
    } else {
        s.to_string()
    }
}

#[test]
fn extract_latex_works() {
    #[track_caller]
    fn t(input: &str, expected: &str) {
        assert_eq!(extract_latex(input), expected);
    }
    t(
        r"
Here's the solution:

```latex
\begin{align*}
x^2 + 2x + 1 = 0
\end{align*}
```
",
        r"\begin{align*}
x^2 + 2x + 1 = 0
\end{align*}",
    );
}
