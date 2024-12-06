use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::{instance::VdPipelineInstance, VdPipelineConfig, VdPipelineResult};

pub struct VdPipelineRunner {
    instances: Vec<VdPipelineInstance>,
    configs: Vec<VdPipelineConfig>,
    src_files: Vec<Arc<String>>,
}

impl VdPipelineRunner {
    pub fn new(
        config_path: impl AsRef<Path>,
        src_file_paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> VdPipelineResult<Self> {
        let configs = VdPipelineConfig::from_yaml_file(config_path)?;
        let src_files: Vec<Arc<String>> = src_file_paths
            .into_iter()
            .map(|p| {
                let content = fs::read_to_string(p).expect("Failed to read source file");
                Arc::new(content)
            })
            .collect();

        let instances = configs
            .iter()
            .flat_map(|config| {
                src_files
                    .iter()
                    .map(move |src| VdPipelineInstance::new(config.clone(), Arc::clone(src)))
            })
            .collect();

        Ok(Self {
            instances,
            configs,
            src_files,
        })
    }
}

impl VdPipelineRunner {
    pub fn run_all_single_threaded(&mut self) -> VdPipelineResult<()> {
        for instance in &mut self.instances {
            instance.run()?;
        }
        Ok(())
    }

    pub fn run_all_multi_threaded(&mut self) -> VdPipelineResult<()> {
        use rayon::prelude::*;

        self.instances
            .par_iter_mut()
            .try_for_each(|instance| instance.run())?;
        Ok(())
    }
}
