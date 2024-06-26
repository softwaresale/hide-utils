use std::path::{Path, PathBuf};
use clap::{Parser};
use hide_utils_core::global_args::GlobalArgs;

/// A simple command line utility for hiding or un-hiding linux files. Provide a file to hide or
/// un-hide and this tool will do just that. A file is hidden if it starts with a "hide character",
/// which is '.' by default. A file is un-hidden when it does not start with the hide-character.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct HiderArgs {
    #[clap(flatten)]
    global_args: GlobalArgs,
    /// force unhide
    #[arg(short, long)]
    unhide: bool,
    /// force hide
    #[arg(short = 'i', long)]
    hide: bool,
    /// Path to the file to un/hide
    file: PathBuf
}

impl HiderArgs {
    pub fn hide_char(&self) -> char {
        self.global_args.hide_char()
    }
    pub fn verbose(&self) -> bool {
        self.global_args.verbose()
    }
    pub fn unhide(&self) -> bool {
        self.unhide
    }
    pub fn hide(&self) -> bool {
        self.hide
    }
    pub fn file(&self) -> &Path {
        &self.file
    }
}
