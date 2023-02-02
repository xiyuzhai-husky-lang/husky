use super::*;
use husky_doc::FilePosition;
use husky_term::Term;

#[derive(Debug, Clone)]
pub enum HoverActionIR {
    Runnable(Runnable),
    Implementation(FilePosition),
    Reference(FilePosition),
    GoToType(Vec<HoverGotoTypeData>),
}

impl HoverActionIR {
    fn new_goto_type_from_targets(_db: &dyn HoverDb, targets: Vec<Term>) -> Self {
        let targets = targets
            .into_iter()
            .filter_map(|_target| {
                todo!()
                // Some(HoverGotoTypeData {
                //     mod_path: render::path(
                //         db,
                //         target.module(db)?,
                //         target.name(db).map(|name| name.to_string()),
                //     ),
                //     nav: target.try_to_nav(db)?,
                // } )
            })
            .collect();
        HoverActionIR::GoToType(targets)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HoverGotoTypeData {
    pub mod_path: String,
    pub nav: NavigationTarget,
}

impl std::fmt::Debug for NavigationTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("NavigationTarget");
        macro_rules! opt {
            ($($name:ident)*) => {$(
                if let Some(it) = &self.$name {
                    f.field(stringify!($name), it);
                }
            )*}
        }
        f.field("file", &self.file)
            .field("full_range", &self.full_range);
        opt!(focus_range);
        f.field("name", &self.name);
        opt!(kind container_name description docs);
        f.finish()
    }
}

impl<'a> dyn HoverDb + 'a {
    fn prepare_hover_actions(&self, actions: &[HoverActionIR]) -> Vec<CommandLinkGroup> {
        actions
            .iter()
            .filter_map(|it| match it {
                HoverActionIR::Implementation(position) => self.show_impl_command_link(position),
                HoverActionIR::Reference(position) => self.show_ref_command_link(position),
                HoverActionIR::Runnable(r) => self.runnable_action_links(r.clone()),
                HoverActionIR::GoToType(targets) => self.goto_type_action_links(targets),
            })
            .collect()
    }

    fn show_impl_command_link(&self, _position: &FilePosition) -> Option<CommandLinkGroup> {
        todo!()
        // if self
        //     .hover_config()
        //     .hover_action_config()
        //     .enable_implementations()
        //     && self
        //         .hover_config()
        //         .client_commands_config()
        //         .show_reference()
        // {
        //     if let Some(nav_data) = self.goto_implementation(*position).unwrap_or(None) {
        //         let uri: lsp_types::Url = position.file().into();
        //         let command = CommandLink::show_references();

        //         return Some(CommandLinkGroup {
        //             commands: vec![command],
        //             ..Default::default()
        //         });
        //     }
        // }
        // None
    }

    fn show_ref_command_link(&self, _position: &FilePosition) -> Option<CommandLinkGroup> {
        todo!()
        // if db.hover_config().hover_action_config().enable_references()
        //     && db.config.client_commands_config().show_reference()
        // {
        //     if let Some(ref_search_res) = db.analysis.find_all_refs(*position, None).unwrap_or(None) {
        //         let uri = action::url(snap, position.file_id);
        //         let line_index = db.file_line_index(position.file_id).ok()?;
        //         let position = action::position(&line_index, position.offset);
        //         let locations: Vec<_> = ref_search_res
        //             .into_iter()
        //             .flat_map(|res| res.references)
        //             .flat_map(|(file_id, ranges)| {
        //                 ranges.into_iter().filter_map(move |(range, _)| {
        //                     action::location(snap, FileRange { file_id, range }).ok()
        //                 })
        //             })
        //             .collect();
        //         let title = action::reference_title(locations.len());
        //         let command = action::command::show_references(title, &uri, position, locations);

        //         return Some(lsp_ext::CommandLinkGroup {
        //             commands: vec![to_command_link(command, "Go to references".into())],
        //             ..Default::default()
        //         });
        //     }
        // }
        // None
    }

    fn runnable_action_links(&self, _runnable: Runnable) -> Option<CommandLinkGroup> {
        todo!()
        // let hover_actions_config = db.config.hover_actions();
        // if !hover_actions_config.runnable() {
        //     return None;
        // }

        // let cargo_spec = CargoTargetSpec::for_file(snap, runnable.nav.file_id).ok()?;
        // if should_skip_target(&runnable, cargo_spec.as_ref()) {
        //     return None;
        // }

        // let client_commands_config = db.config.client_commands();
        // if !(client_commands_config.run_single || client_commands_config.debug_single) {
        //     return None;
        // }

        // let title = runnable.title();
        // let r = action::runnable(snap, runnable).ok()?;

        // let mut group = lsp_ext::CommandLinkGroup::default();

        // if hover_actions_config.run && client_commands_config.run_single {
        //     let run_command = action::command::run_single(&r, &title);
        //     group
        //         .commands
        //         .push(to_command_link(run_command, r.label.clone()));
        // }

        // if hover_actions_config.debug && client_commands_config.debug_single {
        //     let dbg_command = action::command::debug_single(&r);
        //     group.commands.push(to_command_link(dbg_command, r.label));
        // }

        // Some(group)
    }

    fn goto_type_action_links(
        &self,
        _nav_targets: &[HoverGotoTypeData],
    ) -> Option<CommandLinkGroup> {
        todo!()
        // if !db.config.hover_actions().goto_type_def
        //     || nav_targets.is_empty()
        //     || !db.config.client_commands().goto_location
        // {
        //     return None;
        // }

        // Some(lsp_ext::CommandLinkGroup {
        //     title: Some("Go to ".into()),
        //     commands: nav_targets
        //         .iter()
        //         .filter_map(|it| {
        //             action::command::goto_location(snap, &it.nav)
        //                 .map(|cmd| to_command_link(cmd, it.mod_path.clone()))
        //         })
        //         .collect(),
        // })
    }
}
