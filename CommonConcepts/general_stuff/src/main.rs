const AAA: i32=32+32; //Constants are immutable. They can be defined gloabally unlike normal variables which can only be defined inside functions. We use const keyword for declaring constant and the variable name is usually in Uppercase with underscores instead of spaces.
fn main() {
    let _x = 5;
    println!("The value of x is: {AAA}");
    let x = 6; //This is called shadowing, when we use the same variable name as one declared.
    println!("The value of x is: {x}");

    //This program will compile.
    //While we cannot use the same variable declared without 'mut', we can always use the same variable name to declare a new variable.

    let s="This is a streeeeng";
    println!("{s}");

    //s=s.len(); //Will throw compile time error because, even though we can mutate the value of the variable declared by 'mut', we cannot mutate the variable's datatype.
    //Here, s.len() returns an integer containing the lenght of the provided string. But the variable has already been declared with a string so it's datatype is that itself and we cannot assign a value with a different datatype.
    
    //This is where shadowing is useful.
    let k="streeeeeeeeeeng";
    let k=k.len(); //Shadowing(Using the same name as another variable).
    println!("{k}");

}