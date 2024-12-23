use crate::{VdPipelineError, VdPipelineResult};
use std::path::PathBuf;
use std::sync::Arc;

/// Represents a single input unit for the Visored pipeline
///
/// Each input contains the content of a single LaTeX example environment,
/// along with its source file path and index number within the file.
#[derive(Debug)]
pub struct VdPipelineInput {
    /// Path to the source LaTeX file
    pub file_path: PathBuf,
    /// Index of this example within the file (0-based)
    pub index: usize,
    /// Content extracted from the example environment
    pub content: String,
}

impl VdPipelineInput {
    /// Reads a LaTeX file and splits it into multiple inputs based on example environments
    ///
    /// # Arguments
    /// * `file_path` - Path to the LaTeX file to process
    ///
    /// # Returns
    /// * A vector of Arc-wrapped VdPipelineInput, each containing one example
    pub fn read_examples_from_file(file_path: PathBuf) -> VdPipelineResult<Vec<Arc<Self>>> {
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| VdPipelineError::Io(file_path.clone(), e))?;

        // Extract all example environments
        let mut inputs = Vec::new();
        let mut index = 0;

        // Basic string matching for \begin{example} and \end{example}
        let mut current_pos = 0;
        while let Some(start) = content[current_pos..].find("\\begin{example}") {
            let start = current_pos + start;
            if let Some(end) = content[start..].find("\\end{example}") {
                let end = start + end;

                // Extract the content between \begin{example} and \end{example}
                let example_content = content[start + 15..end].trim().to_string();

                inputs.push(Arc::new(Self {
                    file_path: file_path.clone(),
                    index,
                    content: example_content,
                }));

                index += 1;
                current_pos = end + 13; // Move past \end{example}
            } else {
                break; // Malformed LaTeX - missing \end{example}
            }
        }

        Ok(inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_from_file_single_example() {
        let content = r#"\begin{example}
            Test content
        \end{example}"#;
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, content).unwrap();

        let inputs =
            VdPipelineInput::read_examples_from_file(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].index, 0);
        assert_eq!(inputs[0].content, "Test content");
    }

    #[test]
    fn test_read_from_file_multiple_examples() {
        let content = r#"\begin{example}
            First example
        \end{example}
        Some text in between
        \begin{example}
            Second example
        \end{example}"#;
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, content).unwrap();

        let inputs =
            VdPipelineInput::read_examples_from_file(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0].index, 0);
        assert_eq!(inputs[0].content, "First example");
        assert_eq!(inputs[1].index, 1);
        assert_eq!(inputs[1].content, "Second example");
    }

    #[test]
    fn test_read_from_file_empty() {
        let temp_file = NamedTempFile::new().unwrap();

        let inputs =
            VdPipelineInput::read_examples_from_file(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(inputs.len(), 0);
    }

    #[test]
    fn test_read_from_file_no_examples() {
        let content = "Just some LaTeX content without examples";
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, content).unwrap();

        let inputs =
            VdPipelineInput::read_examples_from_file(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(inputs.len(), 0);
    }
}
