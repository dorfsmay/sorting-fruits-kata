use std::fs::{read_dir, File};
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

fn read_list_of_files(folder: &str) -> io::Result<Option<Vec<PathBuf>>> {
    let mut entries: Vec<PathBuf> = vec![];
    for entry in read_dir(folder)? {
        entries.push(entry?.path());
    }
    if entries.is_empty() {
        Ok(None)
    } else {
        Ok(Some(entries))
    }
}

fn read_file(filename: &PathBuf) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;

    let mut lines: Vec<String> = vec![];
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn main() -> io::Result<()> {
    let dir = &"assets";
    let mut list: Vec<String> = vec![];
    if let Some(entries) = read_list_of_files(dir)? {
        println!("\nOrdered lines from files in {}:\n", dir);
        for entry in entries {
            list.extend(read_file(&entry)?);
        }
    }
    list.sort();
    println!("{:?}", list);
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
