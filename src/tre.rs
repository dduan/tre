use crate::cli::Coloring;
use crate::diagram_formatting;
use crate::file_tree::FileType;
use crate::json_formatting;
use crate::output;
use crate::path_finders;
use lscolors::LsColors;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Mode {
    FollowGitIgnore,
    ExcludeHiddenFiles,
    ShowAllFiles,
}

#[derive(Debug, Clone)]
pub struct RunOptions {
    pub editor: Option<Option<String>>,
    pub mode: Mode,
    pub directories_only: bool,
    pub output_json: bool,
    pub root: String,
    pub max_depth: Option<usize>,
    pub exclude_patterns: Vec<Regex>,
    pub coloring: Coloring,
}

pub fn run(option: RunOptions) {
    let directories_only = option.directories_only;
    let max_depth = option.max_depth.unwrap_or(std::usize::MAX);
    let paths: Vec<(String, FileType)> = match option.mode {
        Mode::FollowGitIgnore => {
            path_finders::find_non_git_ignored_paths(&option.root, directories_only, max_depth)
        }
        Mode::ExcludeHiddenFiles => {
            path_finders::find_non_hidden_paths(&option.root, directories_only, max_depth)
        }
        Mode::ShowAllFiles => {
            path_finders::find_all_paths(&option.root, directories_only, max_depth)
        }
    };

    let paths = if option.exclude_patterns.is_empty() {
        paths
    } else {
        paths
            .into_iter()
            .filter(|(path, _)| {
                let mut pattern_iters = option.exclude_patterns.iter();
                !pattern_iters.any(|p| p.is_match(path))
            })
            .collect()
    };

    if option.output_json {
        println!("{}", json_formatting::format_paths(&option.root, paths));
    } else {
        let format_result = diagram_formatting::format_paths(&option.root, paths);
        let lscolors = LsColors::from_env().unwrap_or_default();
        let coloring = match option.coloring {
            Coloring::Never => None,
            Coloring::Always => Some(&lscolors),
            Coloring::Automatic => {
                if atty::is(atty::Stream::Stdout) {
                    Some(&lscolors)
                } else {
                    None
                }
            }
        };
        if let Some(editor) = option.editor {
            output::print_entries(&format_result, true, coloring);
            let editor = if cfg!(windows) {
                editor.unwrap_or_else(|| "".to_string())
            } else {
                editor.unwrap_or_else(|| "$EDITOR".to_string())
            };
            output::create_edit_aliases(&editor, &format_result);
        } else {
            output::print_entries(&format_result, false, coloring);
        }
    }
}
