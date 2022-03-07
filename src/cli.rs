
use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Brad";

    if command == "hello" {
        println!("Hi {}", name);
    }  else if command == "status" {
        println!("What is the status from {}", name );
    } else {
        println!("Wrong command");
    }
}