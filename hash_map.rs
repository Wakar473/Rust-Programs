pub fn run() {
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"), 50);

//use::std::collections::HashMap;
let teams = vec![String::from("Blue") , String::from("Yellow")] ;
let initial_scores = vec![10,50];

let  score: HashMap<_,_> =
teams.into_iter().zip(initial_scores.into_iter()).collect();

//use std::collections::HashMaps;

let field_name = String::from("Favorite color");

let field_value = String::from("blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.insert(String::from("Yellow"),50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);


let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

println!("{:?}", scores)

}