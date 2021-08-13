use {crate::archive, std::path::Path};

#[test]
fn coping_test() {
    let testing_file = Path::new("testing.txt");
    let to = Path::new("testdir/folder/wola.txt");
    match archive::copy_sync(testing_file, to) {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}
