use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use hide_utils_core::global_args::GlobalArgs;

#[derive(Debug, Clone, ValueEnum)]
pub enum OnFailureMode {
    /// Keep the file hidden
    KeepHidden,
    /// Un-hide it regardless
    UnHide
}

impl Display for OnFailureMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnFailureMode::KeepHidden => f.write_str("keep-hidden"),
            OnFailureMode::UnHide => f.write_str("un-hide")
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct WithHideArgs {
    #[clap(flatten)]
    pub global_args: GlobalArgs,
    /// What to do with the hide file if the command exits with a failure
    #[clap(short, long, default_value_t = OnFailureMode::UnHide)]
    pub on_failure: OnFailureMode,
    /// The file to hide
    pub file: PathBuf,
    /// the command to execute while file is hidden
    #[clap(last = true)]
    pub command: Vec<String>
}
