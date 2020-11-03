
pub fn run(){
    let age: u8 = 19;
    let check_id: bool = false;
    let knows_persono_of_age = true;


    if age >= 21 && check_id || knows_persono_of_age {
        println!("Bartender: What would you like to drink");
    }
    else if age < 21 && check_id {
        println!("Come on: Get outta here");
    }
    else{
        println!("Bartender: I will need to see your Id");
    }

    //Short Hand Ifs

    let is_of_age: bool = if age >=18 {true} else {false};
    println!("{}", is_of_age);
}
