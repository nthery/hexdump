extern crate hexdump;

use std::io::Cursor;
use std::str;

fn dump_string(input: &str, width: usize, expected: &str) {
    let mut cin = Cursor::new(input.as_bytes());
    let mut cout = Cursor::new(Vec::new());
    hexdump::dump(&mut cin, &mut cout, width).unwrap();
    assert_eq!(str::from_utf8(cout.get_ref()).unwrap(), expected);
}

#[test]
fn dump_empty_input() {
    dump_string("", 1, "");
}

#[test]
fn dump_one_byte() {
    dump_string("a", 1, "61   a\n");
}

#[test]
fn dump_one_byte_when_width_greater() {
    dump_string("a", 2, "61      a\n");
}

#[test]
fn dump_two_bytes_on_two_lines() {
    dump_string("ab", 1, "61   a\n62   b\n");
}
