use crate::tre::{self, Mode, RunOption};

pub fn get_run_option(args: &[String]) -> RunOption {
    match tre::cli_options().parse(&args[1..]) {
        Ok(matches) => {
            let mode: Mode = if matches.opt_present("v") {
                Mode::Version
            } else if matches.opt_present("h") {
                Mode::Help
            } else if matches.opt_present("a") {
                Mode::ShowAllFiles
            } else if !cfg!(windows) && matches.opt_present("s") {
                Mode::ExcludeHiddenFiles
            } else {
                Mode::FollowGitIgnore
            };

            let directories_only = matches.opt_present("d");
            let output_json = matches.opt_present("j");
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

            let max_depth: Option<usize> = matches.opt_get("l").unwrap_or(None);

            RunOption {
                mode,
                editor,
                directories_only,
                output_json,
                root,
                max_depth,
            }
        }
        Err(_) => RunOption {
            mode: Mode::Help,
            editor: None,
            directories_only: false,
            output_json: false,
            root: None,
            max_depth: None,
        },
    }
}
