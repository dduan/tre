use crate::diagram_formatting;
use crate::file_tree::FileType;
use crate::json_formatting;
use crate::output;
use crate::path_finders;
use getopts::Options;
use lscolors::LsColors;

pub fn cli_options() -> Options {
    let mut opts = Options::new();
    opts.optflag(
        "a",
        "all",
        "Print all files and directories, including hidden ones.",
    );

    opts.optflag("d", "directories", "Only list directories in output.");

    {
        let alias_file_path = if cfg!(windows) {
            r"%TEMP%\tre_aliases_%USERNAME%"
        } else {
            "/tmp/tre_aliases_$USER"
        };

        let default_program = if cfg!(windows) {
            "a default program"
        } else {
            "$EDITOR"
        };
        opts.optflagopt(
            "e",
            "editor",
            &format!(r#"Create aliases for each displayed result in {} and add a number in front of file name to indicate the alias name. For example, a number "42" means an shell alias "e42" has been created. Running "e42" will cause the associated file or directory to be open with {}, or a command specified along with this command."#,
                    alias_file_path,
                    default_program),
            "EDITOR",
        );
    }
    opts.optflag("j", "json", "Output JSON instead of tree diagram.");

    opts.optflagopt(
        "l",
        "limit",
        "Limit display depth file tree output.",
        "DEPTH",
    );

    if !cfg!(windows) {
        opts.optflag(
            "s",
            "simple",
            "Use normal print despite gitignore settings. '-a' has higher priority.",
        );
    }
    opts.optflag("v", "version", "Show version number.");
    opts.optflag("h", "help", "Show this help message.");
    opts
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
    pub directories_only: bool,
    pub output_json: bool,
    pub root: Option<String>,
    pub max_depth: Option<usize>,
}

fn print_help() {
    let brief = format!(
        r#"Usage: tre [path] [option]

Print files, directories, and symlinks in tree form.

Hidden files and those configured to be ignored by git will be (optionally)
ignored.

{}
Path:
    The root path whose content is to be listed. Defaults to "."."#,
        if cfg!(windows) {
            ""
        } else {
            "With correct configuration, each displayed file can have a shell alias created
for it, which opens the file in the default editor or an otherwise specified
command.\n"
        }
    );

    println!("{}", cli_options().usage(&brief));
    println!("Project site: https://github.com/dduan/tre")
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
pub fn run(option: RunOption) {
    let root = &option.root.unwrap_or_else(|| ".".to_string());
    let directories_only = option.directories_only;
    let max_depth = option.max_depth.unwrap_or(std::usize::MAX);
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
            paths = path_finders::find_non_git_ignored_paths(root, directories_only, max_depth);
        }
        Mode::ExcludeHiddenFiles => {
            paths = path_finders::find_non_hidden_paths(root, directories_only, max_depth);
        }
        Mode::ShowAllFiles => {
            paths = path_finders::find_all_paths(root, directories_only, max_depth);
        }
    }

    if option.output_json {
        println!("{}", json_formatting::format_paths(root, paths));
    } else {
        let format_result = diagram_formatting::format_paths(root, paths);
        let lscolors = LsColors::from_env().unwrap_or_default();
        if let Some(editor) = option.editor {
            output::print_entries(&format_result, true, &lscolors);
            let editor = if cfg!(windows) {
                editor.unwrap_or_else(|| "".to_string())
            } else {
                editor.unwrap_or_else(|| "$EDITOR".to_string())
            };
            output::create_edit_aliases(&editor, &format_result);
        } else {
            output::print_entries(&format_result, false, &lscolors);
        }
    }
}
