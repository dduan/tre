use crate::file_tree::FileType;
use crate::formatting;
use crate::output;
use crate::path_finders;
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

#[derive(Debug, Clone)]
pub enum Mode {
    FollowGitIgnore,
    ExcludeHiddenFiles,
    ShowAllFiles,
    Help,
    Version,
}

#[derive(Debug, Clone)]
pub struct RunOption {
    pub editor: Option<Option<String>>,
    pub mode: Mode,
    pub root: Option<String>,
}

fn print_help() {
    println!("> help")
}
fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
pub fn run(option: RunOption) {
    let root = &option.root.unwrap_or(String::from("."));
    let paths: Vec<(String, FileType)>;
    match option.mode {
        Mode::Help => {
            print_help();
            return;
        }
        Mode::Version => {
            print_version();
            return;
        }
        Mode::FollowGitIgnore => {
            paths = path_finders::find_non_git_ignored_paths(&root);
        }
        Mode::ExcludeHiddenFiles => {
            paths = path_finders::find_non_hidden_paths(&root);
        }
        Mode::ShowAllFiles => {
            paths = path_finders::find_all_paths(&root);
        }
    }
    let format_result = formatting::format_paths(root.to_string(), paths);
    output::print_entries(&format_result);

    if let Some(editor) = option.editor {
        let editor = editor.unwrap_or("$EDITOR".to_string());
        output::create_edit_aliases(&editor, &format_result);
    }
}
