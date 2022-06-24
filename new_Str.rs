pub fn  run() {
   // let mut s = String::new();
   // let data = "initial contents";
   // let s =String::from("initial content");
   // let hello = String::from("hello");
   let mut s = String::from("foo");
   s.push_str("bar");
   let mut s1 = String::from("foo");
   let s2 = "bar";
   s1.push_str(s2);
   println!("s2 is {}" , s2);
  

}