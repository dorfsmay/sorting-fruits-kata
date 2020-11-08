use std::fs;

fn read_list_of_files(folder: String) {
    for entry in fs::read_dir(folder).unwrap() {
        println!("{:?}", entry);
    }
}

fn main() {
    let dir = "assets".to_string();
    read_list_of_files(dir);
}
