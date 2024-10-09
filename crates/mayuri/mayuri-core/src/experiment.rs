use crate::*;
use husky_sha_utils::Sha512Output;
use husky_yaml_utils::ordered::OrderedYaml;
use src::MayuriSrc;
use vec_like::ordered_vec_map::OrderedVecPairMap;
use yaml_rust2::Yaml;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Experiment {
    src: ExperimentSrc,
    config: OrderedYaml,
}

pub type ExperimentSrc = OrderedVecPairMap<ExperimentSrcDestinationPath, ExperimentSrcOrigin>;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct ExperimentSrcDestinationPath {
    relative_path: String,
}

impl ExperimentSrcDestinationPath {
    pub fn new(relative_path: String) -> Self {
        Self { relative_path }
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ExperimentSrcOrigin {
    relative_path: String,
    content_sha: Sha512Output,
}

impl ExperimentSrcOrigin {
    pub fn new(relative_path: String, src: &MayuriSrc) -> Self {
        let content_sha = src[&relative_path];
        Self {
            relative_path,
            content_sha,
        }
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }
}

impl Experiment {
    pub(super) fn new(yaml: &Yaml, src: &MayuriSrc) -> Self {
        Self {
            src: yaml["src"]
                .as_hash()
                .expect("expected hash")
                .iter()
                .map(|(k, v)| {
                    (
                        ExperimentSrcDestinationPath::new(
                            k.as_str()
                                .expect("invalid yaml, expected string")
                                .to_string(),
                        ),
                        ExperimentSrcOrigin::new(
                            v.as_str().expect("invalid, expected string").to_string(),
                            src,
                        ),
                    )
                })
                .collect(),
            config: OrderedYaml::new(&yaml["config"]),
        }
    }
}
