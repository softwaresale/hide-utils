use std::path::PathBuf;
use clap::Parser;

/// A simple command line utility for hiding or un-hiding linux files. Provide a file to hide or
/// un-hide and this tool will do just that. A file is hidden if it starts with a "hide character",
/// which is '.' by default. A file is un-hidden when it does not start with the hide-character.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// If turned on, verbose logging is enabled
    #[arg(short, long)]
    verbose: bool,
    /// Character to append/remove from the front of the filename
    #[arg(short = 'c', long, default_value_t = '.')]
    hide_char: char,
    /// force unhide
    #[arg(short, long)]
    unhide: bool,
    /// force hide
    #[arg(short = 'i', long)]
    hide: bool,
    /// The file to hide/unhide
    file: PathBuf,
}

impl Args {
    pub fn hide_char(&self) -> char {
        self.hide_char
    }
    pub fn unhide(&self) -> bool {
        self.unhide
    }
    pub fn hide(&self) -> bool {
        self.hide
    }
    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
