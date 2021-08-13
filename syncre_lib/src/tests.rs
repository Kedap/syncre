use {crate::archive, std::path::Path};

#[test]
fn coping_test() {
    let testing_file = Path::new("testing.txt");
    let to = Path::new("testdir/folder/hello.txt");
    match archive::copy_sync(testing_file, to) {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}

#[test]
fn coping_symbolic_link_test() {
    let file_with_symbolic_link = Path::new("testing0.txt");
    let to = Path::new("testdir/folder_link/hello.txt");
    match archive::copy_sync(file_with_symbolic_link, to) {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}
