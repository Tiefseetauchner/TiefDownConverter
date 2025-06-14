use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

fn main() {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let raw_authors = std::env::var("CARGO_PKG_AUTHORS").unwrap();
    let authors_cleaned = raw_authors
        .split(':')
        .map(|s| s.split('<').next().unwrap().trim())
        .collect::<Vec<_>>()
        .join(", ");
    let date = chrono::Utc::now().format("%B %Y").to_string();

    let dest_path = Path::new("../docs/template/man_metadata.yaml");
    if let Some(parent) = dest_path.parent() {
        create_dir_all(parent).unwrap();
    }

    let mut file = File::create(dest_path).unwrap();
    writeln!(
        file,
        "footer: {}\nheader: {} - {}",
        version, authors_cleaned, date
    )
    .unwrap();

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=../Cargo.toml");
}
