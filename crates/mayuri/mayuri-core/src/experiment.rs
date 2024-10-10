use crate::*;
use config::nemu::NemuConfig;
use husky_sha_utils::Sha512Output;
use husky_yaml_utils::ordered::OrderedYaml;
use makefile::MayuriMakefileExtracted;
use src::MayuriSrc;
use vec_like::ordered_vec_map::OrderedVecPairMap;
use yaml_rust2::Yaml;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Experiment {
    src: ExperimentSrc,
    config: OrderedYaml,
    makefile: MayuriMakefileExtracted,
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
}

impl ExperimentSrcOrigin {
    pub fn new(relative_path: String, src: &MayuriSrc) -> Self {
        Self { relative_path }
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }
}

impl Experiment {
    pub(super) fn new(
        yaml: &Yaml,
        src: &MayuriSrc,
        makefile: MayuriMakefileExtracted,
        nemu_config: &NemuConfig,
    ) -> Self {
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
            makefile,
            config: OrderedYaml::new(&yaml["config"]),
        }
    }
}

impl Experiment {
    fn run_local(&self) {
        todo!()
    }

    /// Copy code from origin to destination
    fn prepare_local(&self, src: &MayuriSrc) -> std::io::Result<()> {
        use std::fs::{self, File};
        use std::io::Write;
        use std::path::Path;

        for (dest, origin) in &self.src {
            let dest_path = Path::new(dest.relative_path());
            let origin_path = Path::new(origin.relative_path());

            // Create parent directories if they don't exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Read the content from the origin file
            let content = &src[origin.relative_path()].content;

            // Write the content to the destination file
            let mut file = File::create(dest_path)?;
            file.write_all(content.as_bytes())?;
        }

        Ok(())
    }
}
