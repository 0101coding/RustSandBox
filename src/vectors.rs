
//Vectors are resizable arrays
pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    
    //Get single value
    println!("Single value  {}", numbers[3]);

    
    //Add to vector
    numbers.push(6);
    numbers.push(8);

    //Get the length of the array
    println!("Vectors Length  {}", numbers.len());

    //Arrays are stack allocated
    println!("Vectors occupies {} bytes", std::mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);


    println!("{:?}", numbers);
    
    //loop and print
    for x in numbers.iter(){
        println!("Number: {}",x);
    }
    
    //loop and mutate
    for x in numbers.iter_mut(){
        *x *= 2
    }

    println!("{:?}", numbers);
}
