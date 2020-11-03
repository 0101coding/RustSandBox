
//Structs - used to create custom data types

pub fn run(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("color: {} {} {} ", c.red, c.blue, c.green);

    let tc = TupleColor(249, 200,0);
    
    println!("color: {} {} {} ", tc.0, tc.1, tc.2);


    let mut p = Person::new("Mary", "Doe");
    
    println!("Person {}", p.full_name());

    p.set_last_name("Margdalene");
    
    println!("Person {}", p.full_name());

    println!("Person Tuple {:?}", p.to_tuple());


}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}


//Tuple Structs
//
struct TupleColor(u8, u8 , u8);


struct Person {
    first_name : String,
    last_name : String
}

impl Person {
    //construct person
    fn new (first: &str, last: &str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }


    //Get full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }


    //Set last name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    //Name to tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
    
}
