use {
    crate::{algorithm::File, archive},
    std::{
        fs,
        path::{Path, PathBuf},
    },
    testdir::testdir,
};

#[test]
fn coping_test() {
    let simple_file = Path::new("testfiles/hello-world.txt");
    let dir: PathBuf = testdir!();
    let to = dir.join("hello-world.txt");

    archive::copy_sync(simple_file, to.as_path()).unwrap();
}

#[test]
fn coping_symbolic_link_test() {
    let file_with_symbolic_link = Path::new("testfiles/link-hello.txt");
    let dir: PathBuf = testdir!();
    let to = dir.join("link-hello.txt");

    archive::copy_sync(file_with_symbolic_link, to.as_path()).unwrap();
}

#[test]
fn coping_directory_test() {
    let directory = Path::new("testfiles/linked");
    let dir: PathBuf = testdir!();
    let to = dir.join("linked");

    archive::copy_sync(directory, to.as_path()).unwrap();
}

#[test]
fn md4_chunks_test() {
    let archive = File::new("testfiles/hello-world.txt".to_string());
    //assert_eq! is no avaliable on vectors
    if vec!["97668ab2f29d0115bd0d1161b9bec520"] != archive.get_sum_chunks() {
        panic!()
    }
}

#[test]
fn copy_sync_ow_test() {
    let simple_file = Path::new("testfiles/hello-world.txt");
    let dir: PathBuf = testdir!();
    let to = dir.join("hello-world.txt");

    archive::copy_sync_ow(simple_file, to.as_path()).unwrap();
}

#[test]
fn sync_dir_test() {
    let dir_sync = Path::new("testfiles");
    let dir: PathBuf = testdir!();
    let to = dir.join("testfiles-sync");
    fs::create_dir_all(to.clone()).unwrap();

    archive::sync_dir(dir_sync, to.as_path()).unwrap();
}

#[test]
fn sync_dir_ow_test() {
    let dir_sync = Path::new("testfiles");
    let dir: PathBuf = testdir!();
    let to = dir.join("testfiles-overwrite");
    fs::create_dir_all(to.clone()).unwrap();

    archive::sync_dir_ow(dir_sync, to.as_path()).unwrap();
}

#[test]
fn synchronize_test() {
    let dir: PathBuf = testdir!();
    let to = dir.join("test-no-exists");

    archive::synchronize("testfiles", to.to_str().unwrap()).unwrap();
}

// comment for erros in tests (from algorithm.rs)
//#[cfg(test)]
//mod tests {
//use super::*;

//#[test]
//fn step_one() {
//todo!();
//}
//}
