use std::env::temp_dir;
use std::io::Write;
use tempfile::tempfile;

fn main() {
    // Create a file insideof `std::env::temp_dir()`
    let mut file = tempfile().unwrap();
    writeln!(file, "Brian was here. Briefly.").unwrap();
}
