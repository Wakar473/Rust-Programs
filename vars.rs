//Variables hold primitive data or oreferences to data
//Variables are immutable by deafault
//Rust is a block-scoped language
   // let name = "Brad";
 // println!("My name is {}",name);
 
 pub fn run() {
     let name = "Wakar";
     let mut age = 22;
     age = 23;
     println!("My name is {} and I am {} year old", name, age);

     //Define constant
     const ID: i32 = 001;
     println!("ID: {}", ID);

     //Assign multiple vars
     let ( my_name, my_age ) = ("Wakar", 23);
     println!("{} is {}", my_name,my_age );
       }
