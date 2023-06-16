use std::io; //Callin/importing the io module from the std library.
use rand::Rng; //Importing the 'Rng' trait from the rand library. 'Rng' defines the methods used to generate random numbers.
use std::cmp::Ordering; //Ordering is a type returned when cmp() is called. Ordering type has multiple varient states as well: Less, Greater and Equal these are the possible states of the ordering type.

fn main() {
    //variable a which can hold 32 bit signed integers.
    let a: i32 =-2; //Here a is immutable.
    let mut a: u32 =3; //Here a is mutable.
    a=4; //'a' being muted.
//Reading input.
    //Creating a variable and initialising it with an empty string.
    let mut str=String::new();
    println!("Please input your guess: ");
    io::stdin() //Calling the stdin() from io module.
        .read_line(&mut str) //Reads user input and appends it to the string given as argument. & is used so that the same memory space used while that variable was created can be used, without making a copy of that variable in another memory address.
        //References, like variables are immutable by default. Hence why we give &mut var than &var.
        //.read_line() returns a 'Result' value which is an enumeration/enunm, which is a type that can be in one of multiple states.
        //Each of these possible states can be called as varients.
        //.read_line() which gives a 'Result' value enum can be in 2 varients. 'Ok' or 'Err'.
        //'Ok' means the function executed succesfully. The value of the succesfully generated function will be stored inside 'Ok' and 'Err' means there was an error and the operation faild, 'Err' will contain information on the error occured.
        //.read_line() appends the read value from input to the given variable and return the number of characters/bytes that was given as input.(if input is '4', it will return 2 and store it in 'Ok' varient. 2 is because the input we actually gave is '4\n' when we press enter, it is read as a newline character.)
        .expect("Failed to take input from std-i");
        //Since .read_line() will return a 'Result' type, and all types has associated functions, this too has an associated function called except.
        //If the returned 'Result' is an 'Ok' varient, the .except("some text") will just print the value stored in 'Ok'.
        //If the returned 'Result' is an 'Err' the .except("Some text") will print the text inside the function and stop the program.
        //If Err is returned ater a .read_line(), it's likely to be caused by the internal os.

    
    println!("The number you guessed: {}",str);
    let n = rand::thread_rng().gen_range(1..=2);
    //The thread_rg() will return a rng(random number generator) instance, which is local to the current thread only. This function itself won't return random numbers. We have to call the function associated to the returned instance, eg: gen_range(1..=100).
    
    match str.cmp(&n) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    //Here cmp() will compare the random number with the number collected from the user and based on the varient of the returned type, it will print the string we provided.
    //aa
}