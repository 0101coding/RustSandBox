
pub fn run(){

    let mut numbers: [i32; 5] = [1,2,3,4,5];
    
    //Get single value
    println!("Single value  {}", numbers[3]);
    
    //Get the length of the array
    println!("Array Length  {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

    println!("{:?}", numbers);
}
