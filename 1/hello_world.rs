fn main(){
 println!("Hello world");
}


// println! calls a Rust macro. If it had called a function instead, 
//it would be entered as println (without the !). 
//For now, you just need to know that using a ! means that you’re calling a macro 
//instead of a normal function and that macros don’t always follow the same rules as functions.