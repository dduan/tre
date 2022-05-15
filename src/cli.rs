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
    pub all: bool,
    /// Use normal print despite gitignore settings. '-a' has higher priority.
    #[cfg(not(target_os = "windows"))]
    #[clap(long, short, parse(from_flag))]
    pub simple: bool,
    /// Only list directories in output.
    #[clap(long, short, parse(from_flag))]
    pub directories: bool,
    /// Create aliases for each displayed result, and add a number in front of file name to
    /// indicate the alias name. For example, a number "42" means an shell alias "e42" has been
    /// created. Running "e42" will cause the associated file or directory to be open with $EDITOR
    /// (or a default program for the file type on Windows), or a command specified along with this
    /// command.
    #[clap(long, short, value_name = "COMMAND")]
    pub editor: Option<Option<String>>,
    /// Output JSON instead of tree diagram.
    #[clap(long, short, parse(from_flag))]
    pub json: bool,
    /// Limit depth of the tree in output.
    #[clap(long, short)]
    pub limit: Option<usize>,
    /// Exclude paths matching a regex pattern. Repeatable.
    #[clap(long, short = 'E', value_name = "PATTERN")]
    pub exclude: Vec<String>,
    #[clap(
        arg_enum,
        long,
        short,
        value_name = "WHEN",
        default_value = "automatic"
    )]
    /// When to color the output. `automatic` means when printing to a terminal, tre will include
    /// colors; otherwise it will disable colors.
    pub color: Coloring,
    #[clap(default_value = ".")]
    pub path: String,
}
