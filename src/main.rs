#![allow(non_snake_case)]

use std::{fs::{self, File}, io::Write};
fn main() -> std::io::Result<()>{
    let mut paths: Vec<_> = fs::read_dir("./notes/").unwrap()
        .map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    let mut output = File::create("output.md")?;

    //TODO: sort files?
    for path in paths {
        let mut contents = fs::read_to_string(path.path())
            .expect("Should have been able to read contents of file");
        contents.push_str("\n---\n");
        output.write_all(contents.as_bytes())?;
        println!("Name: {}", path.path().display());
    }
    Ok(())
}
