//! Hexadecimal and ASCII dumper
//!
//! For example the following text:
//!
//! ```text
//! Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec rhoncus velit ligula, et
//! scelerisque dui dapibus ut.
//! ```
//!
//! becomes:
//!
//! ```text
//! 4C 6F 72 65 6D 20 69 70   Lorem ip
//! 73 75 6D 20 64 6F 6C 6F   sum dolo
//! 72 20 73 69 74 20 61 6D   r sit am
//! 65 74 2C 20 63 6F 6E 73   et, cons
//! 65 63 74 65 74 75 72 20   ectetur
//! 61 64 69 70 69 73 63 69   adipisci
//! 6E 67 20 65 6C 69 74 2E   ng elit.
//! 20 44 6F 6E 65 63 20 72    Donec r
//! 68 6F 6E 63 75 73 20 76   honcus v
//! 65 6C 69 74 20 6C 69 67   elit lig
//! 75 6C 61 2C 20 65 74 20   ula, et
//! 73 63 65 6C 65 72 69 73   sceleris
//! 71 75 65 20 64 75 69 20   que dui
//! 64 61 70 69 62 75 73 20   dapibus
//! 75 74 2E 0A               ut.?
//! ```

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate getopts;

use std::io::prelude::*;
use std::io::ErrorKind;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types.
    error_chain!{}
}

use errors::*;

/// Read from `rd` as much bytes as can fit in `buf`.
/// Return without filling up `buf` on end-of-file or error.
fn read_up_to<'a>(rd: &mut Read, buf: &'a mut [u8]) -> Result<&'a [u8]> {
    let mut nread = 0;
    loop {
        match rd.read(&mut buf[nread..]) {
            Ok(0) => {
                // EOF
                return Ok(&buf[0..nread]);
            }
            Ok(n) => {
                nread += n;
                assert!(nread <= buf.len());
                if nread == buf.len() {
                    return Ok(&buf[0..nread]);
                }
                continue;
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e).chain_err(|| "read error"),
        }
    }
}

/// Print to `wr` `buf` content as hexadecimal and ASCII formatting it to span `width` bytes.
fn print_line(wr: &mut Write, buf: &[u8], width: usize) -> Result<()> {
    assert!(buf.len() <= width);

    for b in buf {
        write!(wr, "{:02X} ", b).chain_err(|| "write error")?;
    }

    for _ in 0..(width - buf.len()) {
        write!(wr, "   ").chain_err(|| "write error")?;
    }

    write!(wr, "  ").chain_err(|| "write error")?;

    for ch in buf.iter().map(|b| *b as char) {
        let ch = if ch.is_control() || !ch.is_ascii() {
            '?'
        } else {
            ch
        };
        write!(wr, "{}", ch).chain_err(|| "write error")?;
    }

    write!(wr, "\n").chain_err(|| "write error")?;
    Ok(())
}

/// Print  to `wr` hexadecimal and ASCII representation of all bytes read from `rd` formatting it
/// with `width` bytes per line.
pub fn dump(rd: &mut Read, wr: &mut Write, width: usize) -> Result<()> {
    let mut buf = vec![0; width];
    loop {
        match read_up_to(rd, &mut buf) {
            Ok(ref b) if b.is_empty() => break,
            Ok(b) => print_line(wr, b, width)?,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
