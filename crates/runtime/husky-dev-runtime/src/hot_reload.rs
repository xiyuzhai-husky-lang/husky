use crate::*;

impl<Task: IsTask> DevRuntime<Task> {
    pub fn hot_reload(&mut self) {
        todo!()
        // CompilerInstance::new(
        //     RelativePathBuf::from_path(&self.config.comptime.package_dir).unwrap(),
        // )
        // .compile_all();
        // self.load_linkages()
    }
}
