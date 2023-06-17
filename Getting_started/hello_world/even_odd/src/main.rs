use std::io;
use std::process;

fn main(){
    
    let mut str_in=String::new();
    io::stdin()
        .read_line(&mut str_in) //Returns Ok or Err varient
        .expect("Err reading input"); //Returns the num of bytes/characters read as input, if the Result type's varient retruned from .read_line() is 'Ok'.

    let n: i32=match str_in.trim().parse() {
        Ok(b)=>b, //Expects the returned value of type, i32.(+ve & -ve integers upto a big number.)
        Err(_) => {
            println!("Error in input, restart the program again."); 
            process::exit(1); //exit() is the associated fn of type process, eventhough it has a semicolon at the end, it is not an expression statement as it does not produce/return a value to be discarded.
        }//If this code block ended with an expression statement, the return type would be '()' which can't be assigned to a variable of type; i32.
        //Expression statements are statements that end in a semicolon(;) and something that produces a value to be discaded(eg: let sum = x + y;)
    };

    match n%2{
        0 => println!("\nGiven input is: Even"),
        1 => println!("\nGiven input is: Odd"),
        _ => println!("Input invalid!!!"), //Wild card statement that covers all other cases of this match operation.
    };

    //println!("{}%2 = {}",n,n%2);
}