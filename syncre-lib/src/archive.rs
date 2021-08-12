use {
    copy_dir::copy_dir,
    std::{fs, path::Path},
};

/// # Copying files and directories, creating directories if necessary
pub fn copy_p(source: &Path, target: &Path) -> Result<(), String> {
    // Filtrer the possibles errors
    if target.exists() && !target.is_dir() {
        return Err("the file already exists".to_string());
    }

    if target.is_dir() {
        match fs::create_dir_all(target) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
    } else {
        let parent = target.parent().unwrap();
        match fs::create_dir_all(parent) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
    }

    if target.is_file() {
        match fs::copy(source, target) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
    } else {
        match copy_dir(source, target) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
    }
    Ok(())
}
