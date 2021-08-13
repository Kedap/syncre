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

use std::path::PathBuf;

pub struct File {
    path: PathBuf,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_one() {
        todo!();
    }
}
