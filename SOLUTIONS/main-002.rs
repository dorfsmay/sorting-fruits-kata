use std::fs;
use std::io;

fn read_list_of_files(folder: String) -> io::Result<()> {
    for entry in fs::read_dir(folder)? {
        println!("{:?}", entry);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let dir = "assets".to_string();
    read_list_of_files(dir)?;
    Ok(())
}
