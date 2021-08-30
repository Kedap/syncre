use {
    crate::{algorithm::File, archive},
    std::{
        path::{Path, PathBuf},
        str,
    },
    testdir::testdir,
};

#[test]
fn coping_test() {
    let simple_file = Path::new("testfiles/hello-world.txt");
    let dir: PathBuf = testdir!();
    let to = dir.join("hello-world.txt");
    match archive::copy_sync(simple_file, to.as_path()) {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}

#[test]
fn coping_symbolic_link_test() {
    let file_with_symbolic_link = Path::new("testfiles/link-hello.txt");
    let dir: PathBuf = testdir!();
    let to = dir.join("link-hello.txt");
    match archive::copy_sync(file_with_symbolic_link, to.as_path()) {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}

#[test]
fn coping_directory_test() {
    let directory = Path::new("testfiles/linked");
    let dir: PathBuf = testdir!();
    let to = dir.join("linked");
    match archive::copy_sync(directory, to.as_path()) {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}

#[test]
fn chunks_test() {
    let archive = (File::new("testfiles/hello-world.txt".to_string()), {
        let file = File::new("testfiles/hello-world.txt".to_string());
        file.contents_bytes
    });
    let file_bytes = archive.1.chunks(2).next().unwrap();
    //let file_bytes = archive.1.chunks(500).next().unwrap();
    assert_eq!(Ok("he"), str::from_utf8(&file_bytes));
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
