mod args;
mod err;
mod renaming;
mod search;

use std::process::ExitCode;
use clap::Parser;
use log::{info, LevelFilter};
use promptly::prompt_default;
use crate::args::{FileCommandArgs, GlobalArgs, HideArgs, HiderSubCommand};
use crate::err::AppError;
use crate::renaming::{file_is_hidden, hide_file, unhide_file};
use crate::search::find_intended_file;

fn main() -> anyhow::Result<ExitCode> {

    let args = GlobalArgs::parse();

    // configure logger
    env_logger::builder()
        .filter_level(if args.verbose() { LevelFilter::Info } else { LevelFilter::Warn })
        .parse_default_env()
        .init();

    match args.command() {
        None => {}
        Some() => {}
    }

    Ok(ExitCode::SUCCESS)
}

fn default_hide_operation(global_args: &GlobalArgs, args: &HideArgs) -> anyhow::Result<ExitCode> {
    // sanity check arguments
    if args.hide() && args.unhide() {
        return Err(AppError::InvalidCommand("cannot specify 'hide' and 'unhide' at the same time".to_string()).into())
    }

    // if the given file doesn't exist, look for a corresponding hidden or unhidden file
    let file = if !args.file().exists() {
        match find_intended_file(args.file(), global_args.hide_char())? {
            None => {
                return Err(AppError::FileDoesNotExist(args.file().to_path_buf()).into());
            }
            Some(possibly_intended) => {
                println!("the path '{}' does not exist. Did you mean '{}'?", args.file().display(), possibly_intended.display());
                let use_possibly_intended = prompt_default("Should we use the other instead:", true)?;
                if use_possibly_intended {
                    possibly_intended
                } else {
                    println!("Cancelling");
                    return Ok(ExitCode::SUCCESS)
                }
            }
        }
    } else {
        args.file().to_path_buf()
    };

    // check out if the file is hidden or not
    let is_hidden = file_is_hidden(&file, global_args.hide_char())?;

    // if we forced a redundant operation, do nothing
    if (is_hidden && args.hide()) || (!is_hidden && args.unhide()) {
        print!("Specified operation was redundant: ");
        if is_hidden {
            println!("file was already hidden")
        } else {
            println!("file was already unhidden")
        }
        return Ok(ExitCode::SUCCESS)
    }

    // otherwise, flip the hidden status of the file
    let new_path = if is_hidden {
        info!("un-hiding file {}", file.display());
        unhide_file(&file, global_args.hide_char())
    } else {
        info!("hiding file {}", file.display());
        hide_file(&file, global_args.hide_char())
    }?;

    // move the files
    info!("renaming from '{}' -> '{}'", file.display(), new_path.display());
    std::fs::rename(file, new_path)
        .map_err(|io_err| AppError::IOError { context: String::from("while renaming file"), error: io_err })?;
    
    Ok(ExitCode::SUCCESS)
}
