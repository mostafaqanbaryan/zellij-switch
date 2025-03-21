use lexopt::{prelude::*, Parser};
use std::{collections::BTreeMap, path::PathBuf};
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {}

register_plugin!(State);

struct Args {
    layout: LayoutInfo,
    session: Option<String>,
    cwd: Option<PathBuf>,
}

fn parse_args(mut parser: Parser) -> Result<Args, lexopt::Error> {
    let mut temp = Args {
        layout: LayoutInfo::File("default".to_string()),
        cwd: None,
        session: None,
    };

    while let Some(arg) = parser.next()? {
        match arg {
            Value(_) => {}
            Short('c') | Long("cwd") => {
                let cwd: String = parser.value()?.parse()?;
                temp.cwd = Option::Some(PathBuf::from(cwd));
            }
            Short('s') | Long("session") => {
                let session: String = parser.value()?.parse()?;
                temp.session = Option::Some(session);
            }
            Short('l') | Long("layout") => {
                let layout: String = parser.value()?.parse()?;
                temp.layout = LayoutInfo::File(format!("{}.kdl", layout));
            }
            _ => {}
        }
    }
    Ok(temp)
}

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let payload = pipe_message.payload.unwrap();
        let parser = lexopt::Parser::from_args(shell_words::split(&payload).unwrap());
        let args = parse_args(parser).unwrap();
        let session_name = args.session.unwrap();

        switch_session_with_layout(Option::Some(session_name.as_str()), args.layout, args.cwd);
        close_self();
        true
    }

    fn update(&mut self, _: Event) -> bool {
        false
    }
}
