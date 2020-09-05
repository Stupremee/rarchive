use rarchive::{Archive, Filter, Format, ReadArchive};

fn main() {
    let mut a = ReadArchive::new();
    a.support_filter(Filter::All);
    a.support_format(Format::All);
    a.open("./archive.zip").expect("failed to open file");

    for entry in a.entries() {
        println!("{}", entry.pathname());
    }
}
