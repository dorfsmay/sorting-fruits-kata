use std::fs;
use std::io;
use std::path::PathBuf;

fn read_list_of_files(folder: &str) -> io::Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(folder)? {
        entries.push(entry?.path());
    }
    Ok(entries)
}

fn main() -> io::Result<()> {
    let dir = &"assets";
    println!("\nFiles inside folder {}:\n", dir);
    for entry in read_list_of_files(dir)? {
        println!("{:?}", entry);
    }
    Ok(())
}
