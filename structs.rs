// Structs  - Used to create custom data types

//Traditional struct
struct color(u8, u8, u8);
//pub fn run() {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

//Tuple Struct
//struct color(u8, u8, u8);

struct person {
    first_name: String,
    last_name: String
}

impl person {

 //construct person
fn new(first: &str, last: &str) ->  person {
    person {
        first_name: first.to_string(),
        last_name: last.to_string()
    }
    }
    //Get  full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();    
    }
    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

}

     pub fn run() {
    //     let mut c = color {
    //         red: 255,
    //         green: 0,
    //         blue: 0,
    //     };

    //     c.red = 200;
    //     println!("Color: {} {} {}", c.red, c.green, c.blue);
    // }

    // let mut c = color(255, 0, 0);
    //  c.2 = 19;
    // println!("color: {} {} {}", c.0, c.1, c.2);
    // }


    let mut p = person::new("kutta", "amit");
    println!("person {}", p.full_name());
    p.set_last_name("williams");
    println!("person {}", p.full_name());
    println!("person Tuple {:?}", p.to_tuple());
     }
     