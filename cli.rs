use std::env;

pub fn wakar() {
    let args: Vec<String> = env::args().collect();
 let command = args[1].clone();
let name = "wakku";
let name1 = "Amit";
 //   println!("Command: {}", command);

 if command =="hello" {
     println!("Hi {}, how are you?", name);
 }
 else if command == "hi" {
     println!("hi {}, i am fine", name1)
 }
}