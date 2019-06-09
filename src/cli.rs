use super::tre::{Mode, RunOption};
use getopts::Options;

pub fn cli_options() -> Options {
    let mut opts = Options::new();
    opts.optflag(
        "a",
        "all",
        "Print all files and directories, including hidden ones.",
    );
    opts.optflag(
        "s",
        "simple",
        "Use normal print despite gitignore settings. '-a' has higher priority.",
    );
    opts.optflagopt(
        "e", "editor", "TODO", // TODO
        "EDITOR",
    );
    opts.optflag("v", "version", "Show version number.");
    opts.optflag("h", "help", "Show this help message.");
    return opts;
}

pub fn get_run_option(args: &Vec<String>) -> RunOption {
    match cli_options().parse(&args[1..]) {
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
