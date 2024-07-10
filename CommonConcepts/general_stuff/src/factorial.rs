use std::io;

fn read() -> i32{

    let n: i32;
    let mut str=String::new();
    loop{
        str.clear(); //Clears the contents of the variable.
        println!("Enter a number:");
        io::stdin()
            .read_line(&mut str) //.read_line(&mut str) actually appends the input to the variable that we already defined with an empty string. It does not overwrite it. There for you need to define the variable inside the loop for it to shadow the variable before, so that it will overwrite the contents of that variable.
            .expect("Error in input");
    
        n=match str.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("\nError. Enter input again: ");
                continue;
            }
        };
        break;
    }
    return n;
}

fn main(){
    let mut a: i32=read();
    let mut fac: i32=1;
    let n=a;

    loop {
        fac=fac*a;
        a=a-1;

        if a==0{
            break;
        }
    }
    println!("{} factorial is: {fac}",n);
}
