//! Common utilites for AOC 2022 problems.
//!
//! Thanks to @kjvalencik for read_stdin, which I'm using instead of providing a filename in args
//! this year, as well as the pub use pattern for anyhow

use std::io;

pub use anyhow::{Context, Error};

/// Read stdin into a string
pub fn read_stdin() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::Read::read_to_string(&mut io::stdin(), &mut buf)?;
    Ok(buf)
}
