// impl DevRuntime {
//     pub fn new(config: RuntimeConfig) -> DevRuntime {
//         todo!()
//         // let mut runtime = Self {
//         //     storage: Default::default(),
//         //     variant: HuskyRuntimeVariant::None,
//         //     feature_interner: Default::default(),
//         //     // comptime
//         //     file_interner: Default::default(),
//         //     word_interner: Default::default(),
//         //     live_docs: Default::default(),
//         //     linkage_table: LinkageTable::new(config.comptime.linkage_table.clone()),
//         //     item_route_store: Default::default(),
//         //     item_route_interner: Default::default(),
//         //     // config
//         //     config,
//         // };
//         // runtime.init();
//         // runtime
//     }

//     fn init(&mut self) {
//         self.hot_reload();
//         let all_main_files = self.all_target_entrances();
//         should_eq!(all_main_files.len(), 1, "config = {:?}", self.config);
//         let all_diagnostics = self.all_diagnostics();
//         if all_diagnostics.len() > 0 {
//             p!(all_diagnostics);
//             panic!("diagnostic errors")
//         }
//         let package = match self.package(self.opt_target_entrance().unwrap()) {
//             Ok(package) => package,
//             Err(error) => {
//                 self.print_diagnostics();
//                 p!(error);
//                 panic!()
//             }
//         };
//         self.variant = HuskyRuntimeVariant::Learning {
//             session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
//         }
//     }
// }
