

pub fn run(){
    let mut count = 0;


    //Infinite Loop
    loop {
      count += 1;
      println!("Number: {}", count);

      if count == 20 {
         break;
      }
   }


    /*
    //While loop (FizzBuzz)
    while count <= 100 {
       if count % 15 == 0 {
           println!("fizzbuzz");
       } else if count % 3 == 0  {
           println!("fizz");
       } else if count % 5 == 0 {
           println!("buzz");
       } else {
           println!("{}", count);
        }
       count = count + 1;
    }
    */

    //For range
    for count in 0..100 {
         if count % 15 == 0 {
           println!("fizzbuzz");
       } else if count % 3 == 0  {
           println!("fizz");
       } else if count % 5 == 0 {
           println!("buzz");
       } else {
           println!("{}", count);
        }
    }
}
