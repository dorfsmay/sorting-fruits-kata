
# Picking up Rust
Material for picking up Rust for experienced developers:

Material from the [RustBridge organisation](https://rustbridge.com/):  
https://intro.rustbridge.com/en/intro/#1

Rustlings: lots of TDD style exercises to practice Rust  
https://github.com/rust-lang/rustlings

# The sorting fruits kata

A progressive kata to review Rust concepts by building a small apps step by step.

Given one fruit name (or fruit type) per line in the files in the assets folder, concatenate them in a single file, "fruits.txt", ordered alphabetically. The full list is very small, therefore it can be kept and manipulated in memory before writing it out to file.

Before using this repository, make sure you run `cargo init .`

# Reference
https://en.wikipedia.org/wiki/Fruit
https://github.com/dorfsmay/sorting-fruits-kata

# Steps
## 001 Print list of files
### Learnings
* printing debug information {:?}  
  https://doc.rust-lang.org/std/fmt/trait.Debug.html
* Introduction to Result and unwrap  
  https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap.html
* for in loops  
  https://doc.rust-lang.org/1.2.0/book/for-loops.html
### Instructions
* create a function that:
    * take a String argument
    * print the list of files from the directory name given as the argument
* unwrap everything for now
* the function just prints, it does not return anything
* In the main function, create a String variable containing "assets"
* Use this variable to call your function

## 002 Result
### Learnings
* Rust Result  
  https://doc.rust-lang.org/std/result/
  https://doc.rust-lang.org/std/fs/fn.read_dir.html
* ?.
  https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
### Instructions
* Remove "unwrap"


## 003 Ownership
### Learnings
* ownership: move, copy
* passing by reference / borrowing
* automatic dereferencing
* str vs String and string literals  
  https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
### Instructions
* Make your main print "Files inside folder _folder-name_ before calling the function 
* Note 1: You can create a reference to string literal, no need to convert to String
* Note 2: note how you do not need to change the type of the variable for read_dir (automatic dereferencing)

## 004 Print from the array
### Learnings
* Rust Vectors and vec! macro
* mutable variables
* Rust guarding against non-initialised variables
### Instructions
* Make the function return a Vector of PathBuff
* Keep using a loop to create the vector, don't jump into a turbo fish yet
* Make main to print them

## 005 Return an Option
### Learnings
* Rust Option  
  https://doc.rust-lang.org/std/option/
* if let
### Instructions
* Add unit tests checkink for Some
* decide if you want to user unwrap for the unit tests, or make tem return a Result
* Make the function returning an Option
* adjust main

## 006 Vector manipulation
### Learnings
* more ways to use modules
* crate prelude  
  https://stackoverflow.com/questions/36384840/what-is-the-prelude
### Instructions
* Add a function to read the content of a file into a vector
* make main concatenate the content of all the files into a single vector
* make main sort the list of words (using the Vector sort)
* make main print the entire list

