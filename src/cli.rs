use crate::tre::{Mode, RunOption};
use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Clone, Debug)]
pub enum Coloring {
    Automatic,
    Always,
    Never,
}

impl core::str::FromStr for Coloring {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "never" => Coloring::Never,
            "always" => Coloring::Always,
            _ => Coloring::Automatic,
        })
    }
}

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Interface {
    /// Print all files and directories, including hidden ones.
    #[clap(long, short, parse(from_flag))]
    all: bool,
    /// Use normal print despite gitignore settings. '-a' has higher priority.
    #[clap(long, short, parse(from_flag))]
    simple: bool,
    /// Only list directories in output.
    #[clap(long, short, parse(from_flag))]
    directories: bool,
    /// Create aliases for each displayed result in {} and add a number in front of file name to
    /// indicate the alias name. For example, a number "42" means an shell alias "e42" has been
    /// created. Running "e42" will cause the associated file or directory to be open with {}, or
    /// a command specified along with this command.
    #[clap(long, short, value_name = "COMMAND")]
    editor: Option<Option<String>>,
    /// Output JSON instead of tree diagram.
    #[clap(long, short, parse(from_flag))]
    json: bool,
    /// Limit depth of the tree in output.
    #[clap(long, short)]
    limit: Option<usize>,
    /// Exclude paths matching a regex pattern. Repeatable.
    #[clap(long, short = 'E', value_name = "PATTERN")]
    exclude: Vec<String>,
    #[clap(long, short, value_name = "WHEN", default_value = "automatic")]
    /// When to color the output (automatic, always, or never). `automatic` means when printing to
    /// a terminal, tre will include colors; otherwise it will disable colors.
    color: Coloring,
    #[clap(default_value = ".")]
    path: String,
}

impl Interface {
    pub fn as_options(self) -> RunOption {
        let mode: Mode = if self.all {
            Mode::ShowAllFiles
        } else if !cfg!(windows) && self.simple {
            Mode::ExcludeHiddenFiles
        } else {
            Mode::FollowGitIgnore
        };

        RunOption {
            editor: self.editor,
            mode,
            directories_only: self.directories,
            output_json: self.json,
            root: self.path,
            max_depth: self.limit,
            exclude_patterns: self
                .exclude
                .iter()
                .filter_map(|p| regex::Regex::new(p).ok())
                .collect(),
            coloring: self.color,
        }
    }
}
