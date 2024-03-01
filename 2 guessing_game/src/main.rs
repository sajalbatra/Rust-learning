use std::io;
use rand::Rng;
use std::cmp::Ordering;
//code to guess the number and compare it 
fn main() {
    let secretnum=rand::thread_rng().gen_range(0..=10);
    println!("Guess the number game");
    println!("the Guesses number is {secretnum}");
    println!("Enter the number");
    let mut guess=String::new();//this line has created a mutable variable that is currently bound to a new, empty instance of a String
    //let decalares a var and it is immutable
    // let mut means we are declaring a variable that is mutable
    //here we created a mutable variable named guess

    io::stdin()
        .read_line(&mut guess)// this readline tells to take input from the user whatever is entered and appends it in the variable guess without overridding anything into it.
        //we are using & to take reference of that code directly without creating copy of that code.
        // we are writing &mut guess to make it mutable
        //readline returns a Result value in form of enum 
        //Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.
        .expect("Failed to read line");
    println!("You guesses {guess}"); 
    // also write it as 
    println!("You guesses {}",guess); 
    

    //Adding more funcationality
    //Rust STL does not support random intergers genrations
    // we will use rand crate and make changes in Cargo.toml
    
    //use to compare the guess n secretnumber but will get error bcz one is str and one is int


    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //Rust allows us to shadow the previous value of guess with a new one
    //and using this we are shadowing the value of guess into it only rather than creating the multiples variable to store the numbers in some other variable
    //u32 is to make all the other data types into string only
    //trim will remove extra spaces
    //parse will remove all the /n from the input
    //parse only works on characters 
    match guess.cmp(&secretnum) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    //loop{...} craetes a infinate loop 
    // type quit to close the loop
    //bcz of parse if the user enters a non-number answer, the program will crash so we are taking advantage of it
    //use break to quit the loop like when user makes a correct guess and add break next to you win the loop will break
}
