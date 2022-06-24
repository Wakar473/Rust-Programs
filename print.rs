pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");
  
    //Basic formatting
    println!("{} is the form {}", "Brad", "Mass");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");


    //Named Arguments
    println!("{name} likes too play {activity}", 
    name= "john", 
    activity = "Baseball"
); 

//Placeholder traits
println!("Binary: {:b} Hex: {:x} octal: {:o}", 10, 10, 10);

//Placeholder for debug trait
println!("{:?}", (12, true, "hello"));

//basic math
println!(" Arun{}", 10 + 10); 
}
 