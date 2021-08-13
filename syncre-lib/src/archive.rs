//! Module archive

use {
    copy_dir::copy_dir,
    std::{
        fs, io,
        io::{Error, ErrorKind},
        path::Path,
    },
};

/// Copying files and directories, creating directories if necessary
///
/// # Example
///
/// ```
/// use std::path::Path;
/// let from = Path::new("dir/dir0/file.txt");
/// let to = Path::new("/usr/app/configs/config.yml");
/// match syncre::archive::copy_sync(from, to) {
///     Err(e) => panic!("{}", e),
///     Ok(v) => v
/// }
/// ```
pub fn copy_sync(source: &Path, target: &Path) -> Result<(), io::Error> {
    // Filtrer the possibles errors
    if target.exists() && !target.is_dir() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "the file already exists",
        ));
    }

    if target.is_dir() {
        if let Err(e) = fs::create_dir_all(target) {
            return Err(e);
        }
    } else {
        let parent = target.parent().unwrap();
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(e);
        }
    }

    if target.is_file() {
        if let Err(e) = fs::copy(source, target) {
            return Err(e);
        }
    } else if let Err(e) = copy_dir(source, target) {
        return Err(e);
    }
    Ok(())
}
