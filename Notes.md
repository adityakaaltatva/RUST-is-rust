Cargo is rust builds system and package manager 

cargo new hello_world       #cargo command for new files
Cargo run
Cargo build 
Cargo check

building your code, downloading the libraries your code depends on, and building those libraries

From <https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html> 





Code : main.rs

Fn main(){
    println!("Hello world");
}
Filename: Cargo.toml


[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]



use std::io;
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude

If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.

=> println! is a macro that prints a string to the screen

// In rust variables are by default immutable 