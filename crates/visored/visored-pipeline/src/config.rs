pub mod stages;

use self::stages::*;
use crate::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdPipelineConfig {
    pub src_file_path: PathBuf,
    pub index: usize,
    pub data: VdPipelineConfigData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VdPipelineConfigData {
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
    pub fn from_yaml_file(path: impl AsRef<Path>) -> VdPipelineResult<Vec<Arc<Self>>> {
        let path = path.as_ref();
        let file =
            std::fs::File::open(path).map_err(|e| VdPipelineError::Io(path.to_path_buf(), e))?;

        let deserializer = serde_yaml::Deserializer::from_reader(file);
        let mut configs = Vec::new();

        for (index, document) in deserializer.enumerate() {
            let data = VdPipelineConfigData::deserialize(document).map_err(|e| {
                VdPipelineError::ConfigParsing(format!("Failed to parse YAML document: {}", e))
            })?;
            configs.push(Arc::new(Self {
                src_file_path: path.to_path_buf(),
                index,
                data,
            }));
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
            Arc::new(VdPipelineConfig {
                src_file_path: PathBuf::from("config-examples/standard.yaml"),
                index: 0,
                data: VdPipelineConfigData {
                    cache_dir: default_cache_dir(),
                    output_name: "baseline".to_string(),
                },
            }),
            Arc::new(VdPipelineConfig {
                src_file_path: PathBuf::from("config-examples/standard.yaml"),
                index: 1,
                data: VdPipelineConfigData {
                    cache_dir: default_cache_dir(),
                    output_name: "standard".to_string(),
                },
            }),
        ]
    );
}
