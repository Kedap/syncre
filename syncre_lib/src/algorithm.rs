//! A naive implementation of the `rsync` algorithm as published at [https://rsync.samba.org/tech_report/tech_report.html](https://rsync.samba.org/tech_report/tech_report.html).
//!
//! Given a `Source` computer with a File, and a `Destination` with a different, yet "similar"
//! File, and supposing that there is a slow communications link between the computers.
//!
//! 1. `Destination` computer splits its `File` into a series of non-overlapping blocks of size
//! `BLOCK_SIZE`
//!
//! 2. For each block, `Destination` computer calculates the `RollingChecksum` (also called
//!    'weak'), and the `StrongChecksum`.
//!
//! 3. `Destination` computer sends the checksums to `Source` computer
//!
//! 4. `Source` searches its `File` to find all blocks of length `BLOCK_SIZE` bytes (at any offset, not just multiples of `BLOCK_SIZE`) that have the same weak and strong checksum as one of the blocks of `Destination` computer. This can be done in a single pass.

use {
    bytes::Bytes,
    md4::{Digest, Md4},
    simd_adler32::Adler32,
    std::{fs, path, path::PathBuf},
};

/// File structure is necessary for the best manipulation of the files with the algorithm
///
/// # Example
///
/// ```
/// use {
///     syncre_lib::algorithm::File,
///     std::path::Path,
/// };
/// let file = File::new("testfiles/hello-world.txt".to_string());
/// assert_eq!(Path::new("testfiles/hello-world.txt").to_path_buf(), file.path);
/// println!("Bytes total: {}", file.bytes);
/// assert!(file.contents_bytes.is_ascii());
/// ```
pub struct File {
    pub path: PathBuf,
    pub bytes: usize,
    pub contents_bytes: Bytes,
}
impl File {
    pub fn new(path: String) -> Self {
        File {
            path: path::Path::new(&path).to_path_buf(),
            bytes: {
                let file = match fs::read(path.clone()) {
                    Ok(v) => v,
                    Err(e) =>
                    /*using panic temporaly*/
                    {
                        panic!("{}", e)
                    }
                };
                let bytes = Bytes::from(file);
                bytes.len()
            },
            contents_bytes: {
                let file = match fs::read(path.clone()) {
                    Ok(v) => v,
                    Err(e) =>
                    /*using panic temporaly*/
                    {
                        panic!("{}", e)
                    }
                };
                Bytes::from(file)
            },
        }
    }

    pub fn get_sum_chunks(&self) -> Vec<String> {
        let mut sums: Vec<String> = Vec::new();
        let bytes = &self.contents_bytes;
        let iter = bytes.chunks(500);
        for chunk in iter {
            sums.push(strong_checksum(chunk))
        }
        sums
    }
}
pub struct Source {
    file: File,
}
pub struct Destination {
    file: File,
}
pub struct FileBlock {}

/// Returns a strong sum (md4) as stated in the rsync algorithm. On String
///
/// # Example
///
/// ```
/// use syncre_lib::algorithm;
/// let bytes = b"hello world from rust :D";
/// println!("{}", algorithm::strong_checksum(bytes));
/// ```
pub fn strong_checksum(bytes: &[u8]) -> String {
    let mut hasher = Md4::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Returns a string of a weak sum as the rsync algorithm says (adler-32 checksum)
///
/// # Example
///
/// ```
/// use syncre_lib::algorithm;
/// let bytes = b"hello world from rust :D";
/// println!("{}", algorithm::rolling_checksum(bytes));
/// ```
pub fn rolling_checksum(bytes: &[u8]) -> String {
    let mut adler = Adler32::new();
    adler.write(bytes);
    adler.finish().to_string()
}
// Moving the tests to tests.rs
