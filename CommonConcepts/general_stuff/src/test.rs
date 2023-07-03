fn main(){
	let sum = -5.098789789; //All floating point datatypes are signed. By default it is 64 bits. 
	println!("{}",sum);
	println!("A");

	print!("B");
	print!("c");
	println!("D");

	eprint!("E");
	eprint!("E");
	eprint!("E");
	eprint!("E\n");

	println!("{subject} {verb} {object}",
	object="the lazy dog",
	subject="the quick brown fox",
	verb="jumps over");

	println!("My name is {0}, {1} {0}", "Bond", "James");
}