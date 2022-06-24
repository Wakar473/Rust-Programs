pub fn run () {
    let str1 = String::from("Hello, world!");
    let str2 = reverse_string(&str1);
    println!("The reverse of string \"{}\" is \"{}\".", str1, str2);
  }
   
  fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
  }