pub mod model_presets;
pub mod model_routing;
pub mod stages;

use self::model_presets::*;
use self::model_routing::*;
use self::stages::*;
use crate::*;
use all_llms::model::AllLlmModel;
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
    pub routing_resolved: VdPipelineModelRoutingResolved,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VdPipelineConfigData {
    // #[serde(default)]
    // stages: VdPipelineStagesConfig,
    #[serde(default = "default_cache_dir")]
    pub cache_dir: RelativePathBuf,
    pub output_name: String,
    pub presets: VdPipelineModelPresets,
    pub routing: VdPipelineModelRouting,
}

fn default_cache_dir() -> RelativePathBuf {
    RelativePathBuf::from(".disk-cache")
}

fn default_true() -> bool {
    true
}

impl VdPipelineConfig {
    pub fn new(src_file_path: PathBuf, index: usize, data: VdPipelineConfigData) -> Self {
        Self {
            src_file_path,
            index,
            routing_resolved: VdPipelineModelRoutingResolved::new(&data.routing, &data.presets),
            data,
        }
    }

    pub fn from_yaml_file(path: impl AsRef<Path>) -> VdPipelineResult<Vec<Arc<Self>>> {
        let path = path.as_ref();
        let file =
            std::fs::File::open(path).map_err(|e| VdPipelineError::Io(path.to_path_buf(), e))?;

        let deserializer = serde_yaml::Deserializer::from_reader(file);
        let mut configs = Vec::new();

        for (index, document) in deserializer.enumerate() {
            let serde_data = VdPipelineConfigData::deserialize(document).map_err(|e| {
                VdPipelineError::ConfigParsing(format!("Failed to parse YAML document: {}", e))
            })?;
            configs.push(Arc::new(Self::new(path.to_path_buf(), index, serde_data)));
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
            Arc::new(VdPipelineConfig::new(
                PathBuf::from("config-examples/standard.yaml"),
                0,
                VdPipelineConfigData {
                    cache_dir: default_cache_dir(),
                    output_name: "baseline".to_string(),
                    presets: vec![VdPipelineModelPreset {
                        name: "gpt-4o".to_string(),
                        model: AllLlmModel::GPT_4O,
                    },],
                    routing: VdPipelineModelRouting {
                        solver: VdPipelineModelSolverRouting {
                            mathematical_reasoning: "gpt-4o".to_string(),
                            mathematical_understanding: "gpt-4o".to_string(),
                            latex_cleaner: "gpt-4o".to_string(),
                        },
                        verifier: VdPipelineModelVerifierRouting {
                            snl_dispatcher: "gpt-4o".to_string(),
                        },
                    },
                },
            )),
            Arc::new(VdPipelineConfig::new(
                PathBuf::from("config-examples/standard.yaml"),
                1,
                VdPipelineConfigData {
                    cache_dir: default_cache_dir(),
                    output_name: "standard".to_string(),
                    presets: vec![
                        VdPipelineModelPreset {
                            name: "gemini-1.5-flash".to_string(),
                            model: AllLlmModel::GEMINI_1_5_FLASH,
                        },
                        VdPipelineModelPreset {
                            name: "gemini-1.5-pro".to_string(),
                            model: AllLlmModel::GEMINI_1_5_PRO,
                        },
                    ],
                    routing: VdPipelineModelRouting {
                        solver: VdPipelineModelSolverRouting {
                            mathematical_reasoning: "gemini-1.5-pro".to_string(),
                            mathematical_understanding: "gemini-1.5-pro".to_string(),
                            latex_cleaner: "gemini-1.5-flash".to_string(),
                        },
                        verifier: VdPipelineModelVerifierRouting {
                            snl_dispatcher: "gemini-1.5-flash".to_string(),
                        },
                    },
                },
            )),
        ]
    );
}
