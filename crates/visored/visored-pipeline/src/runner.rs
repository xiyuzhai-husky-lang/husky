use crate::{
    input::VdPipelineInput, instance::VdPipelineInstance, VdPipelineConfig, VdPipelineResult,
};
use eterned::db::EternerDb;
use std::path::Path;
use std::sync::Arc;
use std::{fs, path::PathBuf};

pub struct VdPipelineRunner<'db> {
    db: &'db EternerDb,
    instances: Vec<VdPipelineInstance>,
    configs: Vec<VdPipelineConfig>,
    src_inputs: Vec<Arc<VdPipelineInput>>,
}

impl<'db> VdPipelineRunner<'db> {
    pub fn new(
        db: &'db EternerDb,
        config_path: impl AsRef<Path>,
        src_file_paths: impl IntoIterator<Item = PathBuf>,
    ) -> VdPipelineResult<Self> {
        let configs = VdPipelineConfig::from_yaml_file(config_path)?;

        let mut src_inputs = vec![];
        for path in src_file_paths {
            let examples = VdPipelineInput::read_examples_from_file(path)?;
            src_inputs.extend(examples);
        }
        let instances = configs
            .iter()
            .flat_map(|config| {
                src_inputs
                    .iter()
                    .map(move |src| VdPipelineInstance::new(config.clone(), Arc::clone(src)))
            })
            .collect();

        Ok(Self {
            db,
            instances,
            configs,
            src_inputs,
        })
    }
}

impl<'db> VdPipelineRunner<'db> {
    pub fn run_all_single_threaded(&mut self) -> VdPipelineResult<()> {
        for instance in &mut self.instances {
            instance.run(self.db)?;
        }
        Ok(())
    }

    pub fn run_all_multi_threaded(&mut self) -> VdPipelineResult<()> {
        use rayon::prelude::*;

        self.instances
            .par_iter_mut()
            .try_for_each(|instance| instance.run(self.db))?;
        Ok(())
    }
}
