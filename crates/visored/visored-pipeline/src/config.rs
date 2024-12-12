pub mod stages;

use self::stages::*;
use crate::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VdPipelineConfig {
    // #[serde(default)]
    // stages: VdPipelineStagesConfig,
    #[serde(default = "default_cache_dir")]
    pub cache_dir: RelativePathBuf,
    pub output_name: String,
}

fn default_cache_dir() -> RelativePathBuf {
    RelativePathBuf::from(".disk-cache")
}

fn default_true() -> bool {
    true
}

impl VdPipelineConfig {
    pub fn from_yaml_file(path: impl AsRef<Path>) -> VdPipelineResult<Vec<Self>> {
        let file = std::fs::File::open(&path)
            .map_err(|e| VdPipelineError::Io(path.as_ref().to_path_buf(), e))?;

        let deserializer = serde_yaml::Deserializer::from_reader(file);
        let mut configs = Vec::new();

        for document in deserializer {
            let config = Self::deserialize(document).map_err(|e| {
                VdPipelineError::ConfigParsing(format!("Failed to parse YAML document: {}", e))
            })?;
            configs.push(config);
        }

        Ok(configs)
    }
}

#[test]
fn vd_pipeline_config_from_yaml_file_works() {
    let configs = VdPipelineConfig::from_yaml_file("config-examples/standard.yaml").unwrap();
    assert_eq!(configs.len(), 2);
    assert_eq!(
        &configs,
        &[
            VdPipelineConfig {
                cache_dir: default_cache_dir(),
                output_name: "baseline".to_string(),
            },
            VdPipelineConfig {
                cache_dir: default_cache_dir(),
                output_name: "standard".to_string(),
            },
        ]
    );
}
