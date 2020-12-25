use std::fs::File;
use std::io::Write;
use tempfile::{tempdir, tempfile};

fn main() {
    println!("std::env::temp_dir = {:?}", std::env::temp_dir());
    // Create a file inside of `std::env::temp_dir()`
    let mut file = tempfile().unwrap();
    writeln!(file, "Brian was here. Briefly.").unwrap();

    // Create a directory inside of `std::env::temp_dir()`
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("temporary-note.txt");
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "Brian was here. Briefly.").unwrap();

    drop(file);
    dir.close().unwrap();
}
