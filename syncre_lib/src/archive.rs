//! Module archive

use {
    copy_dir::copy_dir,
    std::{
        fs, io,
        io::{Error, ErrorKind},
        os::unix,
        path::Path,
    },
};

/// Copying files and directories, creating directories if necessary. The same is copied by the symbolic links (archive mode)
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use syncre_lib::archive;
/// let from = Path::new("dir/dir0/file.txt");
/// let to = Path::new("/usr/app/configs/config.yml");
/// match archive::copy_sync(from, to) {
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
    } else if !source.exists() {
        return Err(Error::new(ErrorKind::NotFound, "the file not exists"));
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

    // Checking if the source is symbolic link
    if let Ok(v) = fs::read_link(source) {
        //Creation of a new symbolic link if the file contains one, the new link will take the file that listed the source
        match unix::fs::symlink(v, target) {
            Err(e) => return Err(e),
            Ok(_) => return Ok(()),
        }
    } else if target.is_file() {
        if let Err(e) = fs::copy(source, target) {
            return Err(e);
        }
    } else if let Err(e) = copy_dir(source, target) {
        return Err(e);
    }
    Ok(())
}
