fn main() {
    let wakar: &'static str = "the quick brown fox jumps over the lazy dog";
   println!("Wakar: {}", wakar);
   
   
 fn fun1<S: std::fmt::Display>(in_fun: S) {
   println!("in fun {} amit", in_fun);
}


   let mut x = 10;
   fun1(&x);
   fun1(&mut x);
   println!("{} , value of x ",x);
   fun1(x);
   fun1("12");
   
   

const USER_LIMIT:i32 = 786;   
  const ARUN:f32 = 3.14;          
  println!("user limit is {}",USER_LIMIT);  
  println!("arun value is {}",ARUN);            
}
