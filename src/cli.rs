use crate::tre::{self, Mode, RunOption};

pub fn get_run_option(args: &Vec<String>) -> RunOption {
    match tre::cli_options().parse(&args[1..]) {
        Ok(matches) => {
            let mode: Mode = if matches.opt_present("v") {
                Mode::Version
            } else if matches.opt_present("h") {
                Mode::Help
            } else if matches.opt_present("a") {
                Mode::ShowAllFiles
            } else if matches.opt_present("s") {
                Mode::ExcludeHiddenFiles
            } else {
                Mode::FollowGitIgnore
            };
            let editor: Option<Option<String>> = if matches.opt_present("e") {
                Some(matches.opt_str("e"))
            } else {
                None
            };
            let root: Option<String> = if matches.free.is_empty() {
                None
            } else {
                Some(matches.free[0].clone())
            };
            RunOption {
                mode: mode,
                editor: editor,
                root: root,
            }
        }
        Err(_) => RunOption {
            mode: Mode::Help,
            editor: None,
            root: None,
        },
    }
}
