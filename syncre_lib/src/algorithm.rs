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
    block_padding::{Padding, ZeroPadding},
    bytes::Bytes,
    md4::{Digest, Md4},
    std::{
        fs,
        io::{Error, ErrorKind},
        path,
        path::PathBuf,
    },
};

pub struct File {
    path: PathBuf,
    bytes: usize,
}
impl File {
    pub fn new(path: String) -> Self {
        File {
            path: path::Path::new(&path).to_path_buf(),
            bytes: {
                let file = match fs::read(path) {
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
        }
    }

    //return the bytes filled in
    // FIXME:
    pub fn padding(&self) -> Result<&mut [u8], Error> {
        if &self.bytes < &500 {
            let contents = match fs::read(&self.path) {
                Ok(v) => v,
                Err(e) =>
                /*using panic temporaly*/
                {
                    panic!("{}", e)
                }
            };
            let number = contents.len();
            let mut buffer = [0xff; 16];
            buffer[..number].copy_from_slice(&contents);
            //E0515, reference to buffer
            Ok(ZeroPadding::pad(&mut buffer, number, &self.bytes - 500).expect("failure to fill"))
        } else {
            //only if is minor that 500 for testing code
            Err(Error::new(ErrorKind::InvalidInput, "no need to fill"))
        }
    }
}
pub struct Source {
    file: File,
}
pub struct Destination {
    file: File,
}
pub struct FileBlock {}
pub struct RollingChecksum {}
pub struct StrongChecksum {}

pub fn md4_sum(bytes: &[u8]) -> String {
    let mut hasher = Md4::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
// Moving the tests to tests.rs
