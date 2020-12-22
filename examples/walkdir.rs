use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("examples") {
        let dir_entry = entry.unwrap();
        println!("{}", dir_entry.path().display());
    }
}
