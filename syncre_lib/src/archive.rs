//! Module archive for manipulate files in archive mode how ```rsync -a```

use std::{
    fs, io,
    io::{Error, ErrorKind},
    path::Path,
};

/// Copying files and directories, creating directories if necessary. The same is copied by the symbolic links (archive mode)
/// This returns an error if: `to` exists and is a directory at the same time, `from` does not exist,
/// something fails when creating the necessary directories and if symbolic links cannot be created
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

    if source.is_dir() {
        if let Err(e) = fs::create_dir_all(target) {
            return Err(e);
        }
    } else {
        let parent = target.parent().unwrap();
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(e);
            }
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
    } else if source.is_file() {
        if let Err(e) = fs::copy(source, target) {
            return Err(e);
        }
    } else if let Err(e) = sync_dir(source, target) {
        return Err(e);
    }
    Ok(())
}

/// It does the same as `copy_sync` but it will not return an error if `to` exists (overwrite)
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use syncre_lib::archive;
/// let from = Path::new("testfiles/hello-world.txt");
/// let to = Path::new("/tmp/testdir/directory_not_exists_overwrite/testfile.txt");
/// match archive::copy_sync_ow(from, to) {
///     Err(e) => panic!("{}", e),
///     Ok(v) => v
/// }
/// ```
pub fn copy_sync_ow(source: &Path, target: &Path) -> Result<(), io::Error> {
    // Filtrer the possibles errors
    if !source.exists() {
        return Err(Error::new(ErrorKind::NotFound, "the file not exists"));
    }

    if source.is_dir() {
        if let Err(e) = fs::create_dir_all(target) {
            return Err(e);
        }
    } else {
        let parent = target.parent().unwrap();
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(e);
            }
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
    } else if source.is_file() {
        if let Err(e) = fs::copy(source, target) {
            return Err(e);
        }
    } else if let Err(e) = sync_dir(source, target) {
        return Err(e);
    }
    Ok(())
}

/// This function synchronizes the contents of one folder to another
/// like copy_dir (<https://crates.io/crates/copy_dir>)
/// This returns an error if: `to` exists and is a directory at the same time, `from` does not exist,
/// something fails when creating the necessary directories and if symbolic links cannot be created
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use syncre_lib::archive;
/// let from = Path::new("testfiles/");
/// let to = Path::new("/tmp/testdir/directory/pro");
/// match archive::sync_dir(from, to) {
///     Err(e) => panic!("{}", e),
///     Ok(v) => v
/// }
/// ```
pub fn sync_dir(src: &Path, dest: &Path) -> Result<(), io::Error> {
    // Filtrer the possibles errors
    if !src.is_dir() || !dest.is_dir() {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "the file is not a directory",
        ));
    } else if !src.exists() || !dest.exists() {
        return Err(Error::new(ErrorKind::NotFound, "the file not exists"));
    }
    let dirs = src.read_dir()?;

    for file in dirs {
        let file = file?;
        let path = file.path();

        let relative_path = path.strip_prefix(src).unwrap();
        let file_path_dest = dest.join(relative_path);
        let file_path_src = src.join(relative_path);

        copy_sync(&file_path_src.as_path(), &file_path_dest.as_path())?;
    }

    Ok(())
}

/// It does the same as `sync_dir` but it will not return an error if `to` exists (overwrite)
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use syncre_lib::archive;
/// let from = Path::new("testfiles/");
/// let to = Path::new("/tmp/testdir/directory-to-overwrite/testfiles");
/// match archive::sync_dir_ow(from, to) {
///     Err(e) => panic!("{}", e),
///     Ok(v) => v
/// }
/// ```
pub fn sync_dir_ow(src: &Path, dest: &Path) -> Result<(), io::Error> {
    // Filtrer the possibles errors
    if !src.is_dir() || !dest.is_dir() {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "the file is not a directory",
        ));
    } else if !src.exists() || !dest.exists() {
        return Err(Error::new(ErrorKind::NotFound, "the file not exists"));
    }
    let dirs = src.read_dir()?;

    for file in dirs {
        let file = file?;
        let path = file.path();

        let relative_path = path.strip_prefix(src).unwrap();
        let file_path_dest = dest.join(relative_path);
        let file_path_src = src.join(relative_path);

        copy_sync_ow(&file_path_src.as_path(), &file_path_dest.as_path())?;
    }

    Ok(())
}

/// This function follows the logic of the command `rsync -a`
/// where the '/' is decisive to know if the destination folder
/// will be added to the name of the source or if they will be called the same
/// If you still don't understand this function well and you have `rsync` installed,
/// the operation is the same as: `rsync -a foo/bar /usr/share/` and `rsync -a foo/bar/
/// /usr/share/bar`
///
/// In both cases the folder '/usr/share/bar' will be created
///
/// # Example
/// ```
/// use syncre_lib::archive;
/// match archive::synchronize("testfiles/linked", "/tmp/no/exists/") {
/// // /tmp/no/exists/linked is created
///     Err(e) => panic!("{}", e),
///     Ok(v) => v
/// }
/// ```
pub fn synchronize(src: &str, dest: &str) -> Result<(), io::Error> {
    let src_path = Path::new(src);

    if src.chars().last().unwrap() == '/' || src_path.is_file() || src_path.is_symlink() {
        copy_sync_ow(Path::new(src), Path::new(dest))?;
    } else {
        let source_name = src_path.file_name().unwrap();
        let dest_final = Path::new(dest).join(source_name);
        copy_sync_ow(Path::new(src), dest_final.as_path())?;
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
