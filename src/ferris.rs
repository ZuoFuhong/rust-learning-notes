extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

pub(crate) fn ferris_says() {
    let out = b"hello fellow Rustacens!";
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap()
}
