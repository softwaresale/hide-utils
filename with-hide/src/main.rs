use std::ffi::OsString;
use std::process::ExitCode;

use clap::Parser;
use duct::cmd;

use hide_utils_core::err::AppError;
use hide_utils_core::renaming::{file_is_hidden, hide_file, unhide_file};

use crate::args::{OnFailureMode, WithHideArgs};

mod args;

fn main() -> anyhow::Result<ExitCode> {

    let args = WithHideArgs::parse();

    // if there are no files, then bail
    if args.command.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    
    // first, hide the file if necessary
    if !(args.file.try_exists()?) {
        return Err(AppError::FileDoesNotExist(args.file).into());
    }

    // if it's not hidden, hide it
    let hidden_file_path = if !(file_is_hidden(&args.file, args.global_args.hide_char())?) {
        hide_file(&args.file, args.global_args.hide_char())?
    } else {
        args.file.clone()
    };
    
    // now, execute the command
    let os_string_segments = args.command.into_iter()
        .map(|cmd_segment| OsString::from(cmd_segment))
        .collect::<Vec<_>>();
    
    let subcommand = cmd(os_string_segments[0].clone(), os_string_segments);
    let output = subcommand.run()?;
    
    // if the command was not a success, determine what to do
    if !output.status.success() {
        match &args.on_failure {
            OnFailureMode::KeepHidden => {
                return Ok(ExitCode::FAILURE)
            }
            _ => {
                // otherwise, we always try to unhide
            }
        }
    }
    
    // un-hide file
    unhide_file(&hidden_file_path, args.global_args.hide_char())?;
    
    // all done
    Ok(ExitCode::SUCCESS)
}
