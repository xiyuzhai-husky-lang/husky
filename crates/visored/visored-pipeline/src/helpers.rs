use std::path::Path;

use crate::{
    instance::VdPipelineInstance,
    runner::{VdPipelineInstanceFile, VdPipelineRunner},
};
use relative_path::{RelativePathBuf, RelativeToError};

const LATEX_DOCUMENT_PREFIX: &'static str = r#"\documentclass{article}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{fvextra}
\fvset{breaklines=true}
\begin{document}"#;

const LATEX_DOCUMENT_SUFFIX: &'static str = r#"\end{document}"#;

const LATEX_MATH_PREFIX: &'static str = r#"\begin{align*}"#;
const LATEX_MATH_SUFFIX: &'static str = r#"\end{align*}"#;

pub struct VdPipelineResultLatexFile {
    pub relative_path: RelativePathBuf,
    pub latex_content: String,
}

impl<'db> VdPipelineRunner<'db> {
    /// Returns pipeline results as LaTeX files.
    ///
    /// Returns pairs of `(relative_path, latex_content)` for each generated file.
    pub fn export_result_latex_files(
        &self,
        parent_dir: impl AsRef<Path>,
    ) -> Result<Vec<VdPipelineResultLatexFile>, RelativeToError> {
        let mut latex_files = Vec::new();
        let parent_dir = parent_dir.as_ref();
        for instance_file in self.instance_files() {
            use relative_path::PathExt;
            let relative_path = instance_file.path.relative_to(parent_dir)?;
            let latex_content = instance_file.result_latex_content(self);
            latex_files.push(VdPipelineResultLatexFile {
                relative_path,
                latex_content,
            });
        }
        Ok(latex_files)
    }
}

impl VdPipelineInstanceFile {
    pub fn result_latex_content(&self, runner: &VdPipelineRunner) -> String {
        let mut latex_content = LATEX_DOCUMENT_PREFIX.to_string();
        for (input, instance_idx_range) in &self.instances {
            for instance_idx in instance_idx_range {
                let instance = &runner[instance_idx];
                instance.write_result_latex_content(&mut latex_content);
            }
        }
        latex_content.push_str(LATEX_DOCUMENT_SUFFIX);
        latex_content
    }
}

impl VdPipelineInstance {
    pub fn write_result_latex_content(&self, latex_content: &mut String) {
        use std::fmt::Write;

        let tracker = self.tracker();
        write!(
            latex_content,
            r#"

Raw solution:
\begin{{verbatim}}
{}
\end{{verbatim}}

\begin{{verbatim}}
Simplified solution:
{}
\end{{verbatim}}

"#,
            tracker.raw_solution, tracker.simplified_solution
        )
        .unwrap();
    }
}
