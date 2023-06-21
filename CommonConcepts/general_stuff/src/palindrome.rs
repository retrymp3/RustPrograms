//To do, fix palindrome error for 10+ characters.

use std::io;
use std::io::Write;

fn read() ->  isize{ //isize is a scalar-integer datatype. It is used to represent a signed integer whose size(32, 64) depend on the computer's architecture. If the computer has a 32bit architecture the integer size will be 32bit.
    let n:  isize;
    let mut num = String::new();
    loop{
        num.clear();
        //Difference between print! and println! macro is that, in println!(), there is a line break at the end of the output, while print!() will not contain a line break.
        print!("Enter input: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        //.flush() instructs the program to display the buffered output immediately. Any output is stored in memory buffer first, before displaying it to the user. When data is written to a stream, like io stream, output is stored in the buffer until it reaches a capacity or a flush() is called.
        //.flush() will immediatly write any pending data to the output stream.
        io::stdin()
            .read_line(&mut num)
            .expect("Error while reading input");

        n=match num.trim().parse(){
            Ok(k) => k,
            Err(_) => {
                println!("\nError, enter input again!\n");
                continue;
            }
        };
        break;
    }
    return n;
}

fn main(){
    let n:  isize=read();
    let mut rem:  isize;
    let mut rev:  isize=0;
    let mut tmp:  isize=n;

    loop{
        rev=rev+(tmp%10);
        rem=tmp/10;

        if rem==0{
            break;
        }
        else{
            tmp=rem;
            //println!("Rev: {}", &rev);
            rev=rev*10;
            //println!("rev: {}", &rev);
        }
    }
    println!("\nOutput: {}", &rev);
    if rev==n{
        println!("Input is a palindrome!");
    }
    else {
        println!("Input is not a palindrome!");
    }
}