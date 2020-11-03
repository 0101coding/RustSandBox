//-----PLEASE NOTE ---------
//variables hold primitive data or references to data
//variables are immutable by default, meaning you can not reassign them
//Rust is a block scoped language
pub fn run() {
   let name = "Mayowa";
   let mut age = 37;

   age =  38;
   println!("My name is {} and I am {}", name, age);

   //Define Constant
   const ID: i32 = 001;
   println!("ID: {}", ID);

   //Assign Multiple vars
   let (my_name, my_age) = ("Mayowa", 33);
   println!("{} is {} years", my_name, my_age);
}
