use std::io; //Callin/importing the io module from the std library.
use rand::Rng; //Importing the 'Rng' trait from the rand library. 'Rng' defines the methods used to generate random numbers.
use std::cmp::Ordering; //Ordering is a type returned when cmp() is called. Ordering type has multiple varient states as well: Less, Greater and Equal these are the possible states of the ordering type.

fn main() {
    //variable a which can hold 32 bit signed integers.
    //let a: i32 =-2; //Here a is immutable.
    //let mut a: u32 =3; //Here a is mutable.
    //a=4; //'a' being muted.
//Reading input.
    //Creating a variable and initialising it with an empty string.
    // let mut str=String::new();
    // println!("Please input your guess: ");
    //io::stdin() //Calling the stdin() from io module.
        //.read_line(&mut str) //Reads user input and appends it to the string given as argument. & is used so that the same memory space used while that variable was created can be used, without making a copy of that variable in another memory address.
        //References, like variables are immutable by default. Hence why we give &mut var than &var.
        //.read_line() returns a 'Result' value which is an enumeration/enunm, which is a type that can be in one of multiple states.
        //Each of these possible states can be called as varients.
        //.read_line() which gives a 'Result' value enum can be in 2 varients. 'Ok' or 'Err'.
        //'Ok' means the function executed succesfully. The value of the succesfully generated function will be stored inside 'Ok' and 'Err' means there was an error and the operation faild, 'Err' will contain information on the error occured.
        //.read_line() appends the read value from input to the given variable and return the number of characters/bytes that was given as input.(if input is '4', it will return 2 and store it in 'Ok' varient. 2 is because the input we actually gave is '4\n' when we press enter, it is read as a newline character.)
        //.expect("Failed to take input from std-i");
        //Since .read_line() will return a 'Result' type, and all types has associated functions, this too has an associated function called except.
        //If the returned 'Result' is an 'Ok' varient, the .except("some text") will just print the value stored in 'Ok'.
        //If the returned 'Result' is an 'Err' the .except("Some text") will print the text inside the function and stop the program.
        //If Err is returned ater a .read_line(), it's likely to be caused by the internal os.

    //Since we are taking a string as input initially, we need to parse it to make it a number which is of the same type as the one we are comparing it to.
    //.trim() will remove the extra characters from the input(white spaces and \n or \r characters.).
    //.parse will return the same enum as .read_line() called Result, the result type will have two variants 'Ok' and 'Err' as we already discussed. And expect will work the same way as well. If 'Ok', output value in 'Ok' else if 'Err', print error msg.
    //It is important to specify the type while defining the variable used to store the parsed value.
    //let n1: i32 = str.trim().parse().expect("Enter a number!!!");

    let n2 = rand::thread_rng().gen_range(1..=100);
    //The thread_rg() will return a rng(random number generator) instance(or type), which is local to the current thread only. This function itself won't return random numbers. We have to call the function associated to the returned instance, eg: gen_range(1..=100).
    
    loop {
        println!("Please input your guess: ");
        let mut str=String::new();

        io::stdin()
            .read_line(&mut str)
            .expect("Faliure!!!");

        //Rust is a statically typed language, which means the compiler should know what type a variable is going to be. The compiler can sometimes guess the variable type based on the assigned value. But when the assigned value is depended on something else, or if the value can be of multiple types, for eg: when we parse() a string into an integer, we need to specify the type, many types can be returned from .parse().
        let n1: i32 = match str.trim().parse() {
            Ok(naaam) => naaam, //match will return 'Ok(naaam)' if the string has succesfully been converted to number. 'naaam' is the value stored inside the 'Ok' varient. In the context of the function used, the 'Ok' varient will store different output inside it. Ok varient of the Result type returned from .read_line() will store the number of bytes/characters of the read input, while 'Ok' of .parse() Result type will return the actual value the function parsed.
            Err(_) => continue, //Here the underscore in Err means that all Err values should match with the Err varient that match returns.
        };

        match n1.cmp(&n2) {
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }

        }
    //Here cmp() will compare the random number with the number collected from the user and based on the varient of the returned type, it will print the string we provided.
    //match is an expression.
    //match has arms. These arms are patterns and it's corresponding code.
    //match will recieve a value and check if it matches with it's arms. If it matches, it will execute the corresponding code.

    //In the above example, str.cmp(&m) will return the type and it's varient(Ordering::Less, Ordering::Greater or Ordering::Equal).
    //If str=30 & n=3, str.cmp will return; Ordering::Greater.
    //match expression will recieve this value and compare with it's arms.
    //It will check with the first arm which is, Ordering::Less. It does not match, so it goes to the next arm.
    //Ordering::Greater this matches with the next arm. Then the println!() macro is executed, and the match expression ends after the first match.
    }
    
}
