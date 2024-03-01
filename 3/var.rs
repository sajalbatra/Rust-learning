//variables are immutable by default in rust that is why we use mut (var mut var_name) to change the variable late on
// with const we cannot use mut
//Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code  
//we can declare a new var with the same name of other var. this is known as shadowing


fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// mut is not same as shadowing 
//we cannot reassign the var without using mul and if we reassign we will get the compile time error  if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed. 
//The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
// let mut spaces = "   ";
//spaces = spaces.len();
// we can get the error bcz we are not allowed to mutate the object type as we are changing its type from string to integer

//Data types 
// Scalar Type: Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
// in integer u32 tells about lentth of integer which is 32 bit
// Length	Signed	Unsigned
// 8-bit	i8	      u8
// 16-bit	i16	      u16
// 32-bit	i32	      u32
// 64-bit	i64	      u64
// 128-bit	i128	  u128
// arch	    isize     usize
// Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). 
// Number literals	Example
// Decimal	       98_222
// Hex	            0xff
// Octal         	0o77
// Binary      	0b1111_0000
// Byte (u8 only) 	b'A'

// Floating point
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

 //operators
fn main() {
    // addition
    let sum = 5 + 10;

  //subtraction
    let difference = 95.5 - 4.3;

   //multiplication
    let product = 4 * 30;

   //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

 // remainder
    let remainder = 43 % 5;
}

//Boolean type
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

// Character type
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}


//Compound Types
//Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

//Tuples
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);  // here we are defining type of each element of tuple

}

fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;  // we are destructuring each element of typle

    println!("The value of y is: {y}");
}

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // accessing the element of tuple using dot operator and at index 0 

    let six_point_four = x.1;  //at index 1

    let one = x.2;
}

//Array 
fn main() {
    let a = [1, 2, 3, 4, 5];
}

// Diff in Array and tuple
//in array all the elements must be of same type and in tuple each element can have different type like int, float...

// Storing string in array
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];


let a: [i32; 5] = [1, 2, 3, 4, 5];// here we are defing array type and its size like it will have 5 elements
let a = [3; 5]; //using this we can assign same value to all 5 elements
// access elements in arrya a[0],a[1]....

// Functions are the block of code. which contains the code and that block is called when we call them
// fn func_name(){...} // function without any parameters
// we call a function like this "another_function();"

// functions with parameters 
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// Expressions in Rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}


// Return type in fn
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

//it has the Return 5 and to make a function return  we must declare their type after the -> 


// control statements in rust 

// if a>b {..} else{...}

// if a {...} a must be boolean and in if a is expecttion a bol otherwise a error will be thrown 
// if a!= b {...} // a not equal to b 


if cond {

}else if cond {

} else if cond {

}else {

}

//in this if condition is false then else if will be check if 1 else if is true then other statements will not bbe checked 


// using if in let 
let cond = true;
let num= if num {4} else {5};
println!({num});


// Loops in Rust 

// loop will be the infinite loop
fn main() {
    loop {
        println!("again!");
    }
}
// we need to stop it by entering anything

// while loop
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// for loop 
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}