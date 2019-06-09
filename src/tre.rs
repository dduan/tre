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

pub fn run(option: RunOption) {}
