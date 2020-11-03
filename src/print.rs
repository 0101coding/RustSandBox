pub fn run(){
    //Print to console
    println!("Hello from the print rs file");

    println!("Number {}", 1);

    println!("{} is from {}", "Mayowa", "Heaven");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Mayowa", "Heaven", "Play");
    //Named Arguments
    println!("{name} like to play {activity}", name = "John", activity = "BaseBall");

    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10,10);

    //Placeholder for Debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}
