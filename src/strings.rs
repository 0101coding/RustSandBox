//Primitve str = Immutable fixed-length string somewhere in memory
//String = Growable, heap allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let hello = "hello";
    let mut new_string = String::from("Another Hello ");

    println!("{}", hello);
    println!("{}", new_string);

    new_string.push('W');

    new_string.push_str("orld");
    //Capacity
    println!("Capacity {}", new_string.capacity());
    println!("Is Empty {}", new_string.is_empty());
    println!("Contains 'World' {}", new_string.contains("World"));
    println!("Replace: {}", new_string.replace("World", "There"));
    println!("{}", new_string);
   
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}" , s);
    
    //Assertion testing
    assert_eq(2, s.len());
    assert_eq(10, s.capacity());

    
}
