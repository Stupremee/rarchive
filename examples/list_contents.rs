use rarchive::{Archive, ReadArchive};

const ARCHIVE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/archive.zip");

fn main() {
    let mut a = ReadArchive::from_file(ARCHIVE).expect("failed to open file");
    a.open(ARCHIVE).unwrap();

    for entry in a.entries() {
        println!("{}", entry.pathname());
    }
}
