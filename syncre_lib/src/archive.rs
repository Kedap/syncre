//! Module archive

use {
    copy_dir::copy_dir,
    std::{
        fs, io,
        io::{Error, ErrorKind},
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
/// let from = Path::new("testfiles/hello-world.txt");
/// let to = Path::new("/tmp/testdir/directory_not_exists/testfile.txt");
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

        #[cfg(target_family = "windows")]
        match create_link_windows(v.as_path(), target) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }

        #[cfg(target_family = "unix")]
        match create_link_unix(v.as_path(), target) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
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

/// Make syslink on windows, either directory or file (only on windows)
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use syncre_lib::archive;
/// let orginal = Path::new("testfiles/linked/hello-windows.txt");
/// let link = Path::new("hello-windows-syslink.txt");
/// match archive::create_link_windows(original, link) {
///     Err(e) => panic!("{}", e),
///     Ok(v) => v
/// }
/// ```
#[cfg(target_family = "windows")]
pub fn create_link_windows(orginal: &Path, link: &Path) -> Result<(), io::Error> {
    use std::os::windows::fs;
    if orginal.is_file() {
        match fs::symlink_file(orginal, link) {
            Ok(_v) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        match fs::symlink_dir(orginal, link) {
            Ok(_v) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Make syslink on unix, either directory or file (only in unix)
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use syncre_lib::archive;
/// let original = Path::new("testfiles/linked/hello-link.txt");
/// let link = Path::new("hello-unix-syslink.txt");
/// match archive::create_link_unix(original, link) {
///     Err(e) => panic!("{}", e),
///     Ok(v) => v
/// }
/// ```
#[cfg(target_family = "unix")]
pub fn create_link_unix(orginal: &Path, link: &Path) -> Result<(), io::Error> {
    use std::os::unix::fs;
    match fs::symlink(orginal, link) {
        Ok(_v) => Ok(()),
        Err(e) => Err(e),
    }
}
