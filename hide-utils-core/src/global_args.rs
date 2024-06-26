use clap::Args;

/// These are arguments that are likely to be used in every single hider-utils binary
#[derive(Debug, Args)]
pub struct GlobalArgs {
    /// If turned on, verbose logging is enabled
    #[arg(short, long)]
    verbose: bool,
    /// Character to append/remove from the front of the filename
    #[arg(short = 'c', long, default_value_t = '.')]
    hide_char: char,
}

impl GlobalArgs {
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn hide_char(&self) -> char {
        self.hide_char
    }
}
