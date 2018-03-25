///! Print to standard output hexadecimal and ASCII dump of files given on command-line.  Dump
///! standard input if the command-line is empty.

#[macro_use]
extern crate error_chain;
extern crate getopts;
extern crate hexdump;

use std::env;
use std::fs::File;
use std::option::Option;
use std::io;
use std::io::BufReader;
use getopts::Options;
use hexdump::errors::*;

const DEFAULT_BYTES_PER_LINE: usize = 8;

fn print_usage(opts: &Options) {
    let brief = "Usage: hexdump [options] files...";
    print!("{}", opts.usage(&brief));
}

struct CmdLine {
    width: usize,
    files: Vec<String>,
}

fn parse_cmdline() -> Result<Option<CmdLine>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut opts = Options::new();
    opts.optopt("w", "", "set number of bytes per line", "COUNT");
    opts.optflag("h", "help", "print this help");

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(e) => bail!(e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&opts);
        return Ok(None);
    }

    let width = if let Some(s) = matches.opt_str("w") {
        match s.parse::<usize>() {
            Ok(n) => n,
            Err(e) => bail!("bad argument passed to -w: {}", e),
        }
    } else {
        DEFAULT_BYTES_PER_LINE
    };

    Ok(Some(CmdLine {
        width,
        files: matches.free,
    }))
}

/// Dump all files on command-line or standard input.
fn do_main() -> Result<()> {
    if let Some(cmdline) = parse_cmdline()? {
        let stdout = io::stdout();
        if cmdline.files.is_empty() {
            let stdin = io::stdin();
            hexdump::dump(&mut stdin.lock(), &mut stdout.lock(), cmdline.width)?;
        } else {
            for path in cmdline.files {
                let f = File::open(&path).chain_err(|| format!("cannot open {}", &path))?;
                let mut reader = BufReader::new(f);
                hexdump::dump(&mut reader, &mut stdout.lock(), cmdline.width)?;
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = do_main() {
        eprintln!("error: {}", e);
        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }
        ::std::process::exit(1);
    }
}
