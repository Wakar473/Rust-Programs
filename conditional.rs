//Conditional - used to check the condition of a something and act on the result

pub fn run() {
    let age = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;


    //if/else
    if age >= 21 && check_id ||knows_person_of_age {
        println!("Bartender:whould you like to drink?");
    }else if age < 21 && check_id {
        println!("Bartender:sorry you have to take permisson to your parents")
    }else {

        println!("Bartender:I'll need to check your adhar which is given by modi");
    }


    //Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)
    }

    

