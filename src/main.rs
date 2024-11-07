use std::{collections::BTreeMap, path::PathBuf};
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let session_name = pipe_message.payload.unwrap().to_string();
        let collection: Vec<&str> = session_name.split("::").collect::<Vec<&str>>().clone();
        let session_name = collection[0];
        let mut layout_name = "default".to_string();
        let mut cwd = None;

        if collection.len() >= 2 {
            cwd = Some(PathBuf::from(collection[1]));
        }
        if collection.len() == 3 {
            layout_name = format!("{}.kdl", collection[2]);
        }

        let layout = LayoutInfo::File(layout_name);
        switch_session_with_layout(Some(&session_name), layout, cwd);
        close_self();
        true
    }

    fn update(&mut self, _: Event) -> bool {
        false
    }
}
