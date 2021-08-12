use {
    copy_dir::copy_dir,
    std::{
        fs, io,
        io::{Error, ErrorKind},
        path::Path,
    },
};

/// # Copying files and directories, creating directories if necessary
pub fn copy_p(source: &Path, target: &Path) -> Result<(), io::Error> {
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
