use std::fs;
use std::io;

fn read_list_of_files(folder: &str) -> io::Result<()> {
    for entry in fs::read_dir(folder)? {
        println!("{:?}", entry);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let dir = &"assets";
    println!("\nFiles inside folder {}:\n", dir);
    read_list_of_files(dir)?;
    Ok(())
}
