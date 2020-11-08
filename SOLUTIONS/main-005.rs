use std::fs;
use std::io;
use std::path::PathBuf;

fn read_list_of_files(folder: &str) -> io::Result<Option<Vec<PathBuf>>> {
    let mut entries: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(folder)? {
        entries.push(entry?.path());
    }
    if entries.is_empty() {
        Ok(None)
    } else {
        Ok(Some(entries))
    }
}

fn main() -> io::Result<()> {
    let dir = &"assets";
    if let Some(entries) = read_list_of_files(dir)? {
        println!("\nFiles inside folder {}:\n", dir);
        for entry in entries {
            println!("{:?}", entry);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_dir_returns_some() {
        assert!(read_list_of_files("assets").unwrap().is_some());
    }
    #[test]
    fn read_dir_returns_none() {
        assert!(read_list_of_files("empty").unwrap().is_none());
    }
}
