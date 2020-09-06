use rarchive::ReadArchive;

fn main() {
    let mut a = ReadArchive::from_file("./archive.zip").expect("failed to open file");

    for entry in a.entries() {
        println!("{}", entry.pathname());
    }
}
