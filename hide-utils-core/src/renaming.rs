use std::path::{Path, PathBuf};
use crate::err::AppError;

/// determine if a file is hidden or unhidden
pub fn file_is_hidden(path: &Path, hide_char: char) -> Result<bool, AppError> {
    let Some(filename) = path.file_name() else {
        return Err(AppError::NoFileName)
    };

    let Some(str_rep) = filename.to_str() else {
        return Err(AppError::FileNameNotUnicode)
    };

    str_rep.chars().nth(0)
        .map(|first_char| first_char == hide_char)
        .ok_or(AppError::NoFileName)
}

/// pull a filename from the given path
pub fn path_filename(path: &Path) -> Result<&str, AppError> {
    let Some(filename) = path.file_name() else {
        return Err(AppError::NoFileName)
    };

    let Some(str_rep) = filename.to_str() else {
        return Err(AppError::FileNameNotUnicode)
    };

    Ok(str_rep)
}

/// Take an un-hidden path and hide it at the file system level. If the path is already hidden,
/// then do nothing
pub fn hide_file(path: &Path, hide_char: char) -> Result<PathBuf, AppError> {
    if file_is_hidden(path, hide_char)? {
        return Ok(path.to_path_buf());
    }
    
    let dest_path = hide_file_path(path, hide_char)?;
    std::fs::rename(path, &dest_path)
        .map_err(|err| AppError::IOError { error: err, context: "rename".to_string() })?;
    
    Ok(dest_path)
}

/// take an unhidden file and create a hidden path version
pub fn hide_file_path(path: &Path, hide_char: char) -> Result<PathBuf, AppError> {
    let parent_path = path.parent();
    let filename = path_filename(path)?;
    let hidden_filename = format!("{}{}", hide_char, filename);
    if let Some(parent_path) = parent_path {
        Ok(parent_path.join(hidden_filename))
    } else {
        Ok(PathBuf::from(hidden_filename))
    }
}

/// Take a hidden path and un-hides it at the file system level. If the path is already un-hidden,
/// then do nothing
pub fn unhide_file(path: &Path, hide_char: char) -> Result<PathBuf, AppError> {
    if !file_is_hidden(path, hide_char)? {
        return Ok(path.to_path_buf());
    }

    let dest_path = unhide_file_path(path, hide_char)?;
    std::fs::rename(path, &dest_path)
        .map_err(|err| AppError::IOError { error: err, context: "rename".to_string() })?;

    Ok(dest_path)
}

/// take a hidden file and make it unhidden
pub fn unhide_file_path(path: &Path, hide_char: char) -> Result<PathBuf, AppError> {
    let parent_path = path.parent();
    let filename = path_filename(path)?;
    let Some(unhidden_filename) = filename.strip_prefix(&hide_char.to_string()) else {
        panic!("We didn't strip the hide prefix from a hidden file")
    };

    if let Some(parent_path) = parent_path {
        Ok(parent_path.join(unhidden_filename))
    } else {
        Ok(PathBuf::from(unhidden_filename))
    }
}

/// given a file path, hide it if it's un-hidden, or un-hide it if it's hidden 
pub fn auto_transition_file(general_path: &Path, hide_char: char) -> Result<(), AppError> {
    let destination_path = if file_is_hidden(general_path, hide_char)? {
        unhide_file_path(general_path, hide_char)
    } else {
        hide_file_path(general_path, hide_char)
    }?;

    std::fs::rename(general_path, destination_path)
        .map_err(|err| AppError::IOError { error: err, context: "rename".to_string() })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::err::AppError;
    use crate::renaming::{file_is_hidden, hide_file_path, path_filename, unhide_file_path};

    #[test]
    fn hide_file_adds_hide_char_without_parent() {
        let path = PathBuf::from("file.txt");
        let expected_path = PathBuf::from(".file.txt");
        let actual_path = hide_file_path(&path, '.').expect("Hide should succeed");
        assert_eq!(expected_path, actual_path);
    }

    #[test]
    fn unhide_file_removes_hide_char_without_parent() {
        let path = PathBuf::from(".file.txt");
        let expected_path = PathBuf::from("file.txt");
        let actual_path = unhide_file_path(&path, '.').expect("Unhide should succeed");
        assert_eq!(expected_path, actual_path);
    }

    #[test]
    fn hide_file_adds_hide_char_with_parent() {
        let path = PathBuf::from("/some/path/to/file.txt");
        let expected_path = PathBuf::from("/some/path/to/.file.txt");
        let actual_path = hide_file_path(&path, '.').expect("Hide should succeed");
        assert_eq!(expected_path, actual_path);
    }

    #[test]
    fn unhide_file_removes_hide_char_with_parent() {
        let path = PathBuf::from("/some/path/to/.file.txt");
        let expected_path = PathBuf::from("/some/path/to/file.txt");
        let actual_path = unhide_file_path(&path, '.').expect("Unhide should succeed");
        assert_eq!(expected_path, actual_path);
    }

    #[test]
    fn get_filename_works() {
        let path = PathBuf::from("/some/path/to/file.txt");
        let filename = "file.txt";
        let actual_filename = path_filename(&path).expect("Should work");
        assert_eq!(filename, actual_filename);
    }

    #[test]
    fn get_filename_fails_with_no_filename() {
        let path = PathBuf::from("/");
        let actual_filename = path_filename(&path).expect_err("Should fail with error");
        let AppError::NoFileName = actual_filename else {
            panic!("Should have failed with no filename")
        };
    }
    
    #[test]
    fn file_is_hidden_detects_hidden_file() {
        let path = PathBuf::from("/hello/.world/");
        assert!(file_is_hidden(&path, '.').expect("should not fail"));
    }

    #[test]
    fn file_is_hidden_detects_hidden_file_special_character() {
        let path = PathBuf::from("/hello/~world/");
        assert!(file_is_hidden(&path, '~').expect("should not fail"));
    }

    #[test]
    fn file_is_hidden_detects_unhidden_file() {
        let path = PathBuf::from("/hello/world/");
        assert!(!file_is_hidden(&path, '.').expect("should not fail"));
    }

    #[test]
    fn file_is_hidden_detects_unhidden_file_special_character() {
        let path = PathBuf::from("/hello/world/");
        assert!(!file_is_hidden(&path, '~').expect("should not fail"));
    }
}
