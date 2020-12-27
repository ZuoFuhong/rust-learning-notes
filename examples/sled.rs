fn main() {
    let db: sled::Db = sled::open("my_db").unwrap();
    db.insert(b"name", b"gina").unwrap();
    assert_eq!(&db.get(b"name").unwrap().unwrap(), b"gina");

    // Atomic compare-and-swap
    db.compare_and_swap(b"name", Some(b"gina"), Some(b"rona"))
        .unwrap()
        .unwrap();

    db.remove(b"name").unwrap();
    assert_eq!(db.get(b"name"), Ok(None));
}
