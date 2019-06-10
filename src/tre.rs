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
        "e",
        "editor",
        r#"Create aliases for each displayed result in /tmp/tre_aliases_$USER and add a number in front of file name to indicate the alias name. For example, a number "42" means an shell alias "e42" has been created. Running "e42" will cause the associated file or directory to be open with $EDITOR, or a command specified along with this command."#,
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
    let brief = r#"Usage: tre [path] [option]

Print files, directories, and symlinks in tree form.

Hidden files and those configured to be ignored by git will be (optionally)
ignored.

With correct configuration, each displayed file can have a shell alias created
for it, which opens the file in the default editor or an otherwise specified
command.

Path:
    The root path whose content is to be listed. Defaults to "."."#;

    println!("{}", cli_options().usage(brief));
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

    if let Some(editor) = option.editor {
        output::print_entries(&format_result, true);
        let editor = editor.unwrap_or("$EDITOR".to_string());
        output::create_edit_aliases(&editor, &format_result);
    } else {
        output::print_entries(&format_result, false);
    }
}
