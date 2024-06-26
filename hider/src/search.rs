use std::path::{Path, PathBuf};
use crate::err::AppError;
use crate::renaming::{file_is_hidden, hide_file, unhide_file};

/// looks for the file the user might have meant. If the user provided a "hidden" file and it doesn't
/// exist, look for the unhidden version of that path
pub fn find_intended_file(provided_path: &Path, hide_char: char) -> Result<Option<PathBuf>, AppError> {
    // figure out if the provided path was hidden or not
    let possibly_intended_path = if file_is_hidden(provided_path, hide_char)? {
         unhide_file(provided_path, hide_char)
    } else {
        hide_file(provided_path, hide_char)
    }?;
    
    possibly_intended_path.try_exists()
        .map(move |exists| if exists {
            Some(possibly_intended_path)
        } else {
            None
        })
        .map_err(|io| AppError::IOError { context: "while trying to look up possibly intended file".to_string(), error: io })
}
