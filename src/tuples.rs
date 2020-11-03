//Tuples group together values of different values
//Max 12 elements
pub fn run() {
    let person:(&str,&str,i8) = ("Mayowa", "Mars", 40);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
