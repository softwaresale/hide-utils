use std::path::{Path, PathBuf};
use clap::{Args, Parser, Subcommand};

/// A simple command line utility for hiding or un-hiding linux files. Provide a file to hide or
/// un-hide and this tool will do just that. A file is hidden if it starts with a "hide character",
/// which is '.' by default. A file is un-hidden when it does not start with the hide-character.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct GlobalArgs {
    /// If turned on, verbose logging is enabled
    #[arg(short, long)]
    verbose: bool,
    /// Character to append/remove from the front of the filename
    #[arg(short = 'c', long, default_value_t = '.')]
    hide_char: char,
    /// What operation to take
    #[clap(subcommand)]
    command: Option<HiderSubCommand>
}

impl GlobalArgs {
    pub fn hide_char(&self) -> char {
        self.hide_char
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn command(&self) -> &Option<HiderSubCommand> {
        &self.command
    }
}

#[derive(Debug, Subcommand)]
pub enum HiderSubCommand {
    /// Default operation. If this given file is hidden, un-hide it. If it is not hidden, hide it.
    Hide(HideArgs),
    /// hide a file, execute a program, and then unhide the file when it's done
    WithHide(WithHiddenArgs)
}

pub trait FileCommandArgs {
    fn file(&self) -> &Path;
}

#[derive(Debug, Args)]
pub struct HideArgs {
    /// force unhide
    #[arg(short, long)]
    unhide: bool,
    /// force hide
    #[arg(short = 'i', long)]
    hide: bool,
    /// Path to the file to un/hide
    file: PathBuf
}

impl HideArgs {
    pub fn unhide(&self) -> bool {
        self.unhide
    }
    pub fn hide(&self) -> bool {
        self.hide
    }
}

impl FileCommandArgs for HideArgs {
    fn file(&self) -> &Path {
        &self.file
    }
}

#[derive(Debug, Args)]
pub struct WithHiddenArgs {
    /// Path to the file to hide
    file: PathBuf,
    /// the command to execute while the given file is hidden
    #[clap(last = true)]
    command: Vec<String>
}

impl FileCommandArgs for WithHiddenArgs {
    fn file(&self) -> &Path {
        &self.file
    }
}
