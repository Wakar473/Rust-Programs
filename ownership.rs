//The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
// The Rust language gives you control over your memory usage in the same way as other systems programming languages, 
//but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

//Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book. 
//Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.


// fn main() {
// //ownership
// //each value in rust has a variable that's called its owner.
// //there can only be one owner at a time.
// //when the owner goes out of scope, the value will bbe dropped.

// //s is not valid here, its not yet declared
// let s: String = Stirgn::from("hello");  //s is valid from this point forward
// takes_ownership(some_string: s);
// println!("{}", s);

// //do stuff with s
// let x: i32 = 5;
// makes_copy(some_integer: x);
// println!("{}", x);
// }


// fn takes_ownership(some_string: String) {
//     println!("{}", some_string); 
// //this scope is now over, and s is no longer valid
// }
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
