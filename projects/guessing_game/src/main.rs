use std::io;
//input output library comes from a standard library std.
//so what i did taking io into scope using use.
//the set of library comes by default is called prelude
fn main(){
	println!("Guess the fucking number: ");
	let mut guess=String::new();
//let is used for creating a variable like "let rhyme= 5" 
//by default variables are immutable means once it is set, it wont
//be changed. so "mut" is used to make it mutable or changing
//new is an associated function of "STRING" type. 
//indicated by ::. so this new function creates new empty string
//an associated function is what implemented on a type
//tldr: the line meant it has created a mutable variable named guess
//which is bound to a new, empty instance of a string type.
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read your value.");
//we could've write std::io::stdin, yeah its like using namespace std
	println!("you guessed: {guess}");
//read line is a method in standard input handle to get the input
//its like scanf in c, cin in cpp and input in python.
//we pass the string as an argument. & means a reference. 
//references are immutable by default and therefore we are using &mut
		
//note that the method all can be written in one line.
//see the semi-colon at the end of this line.
//it is wise to break lines whenever method.name() comes.
//read_line creates an enumeration named "result". what it does
//this enumeration creates a status, 1. ok means successful
//or err means unsuccessful.
//the values of "RESULT" type create a method named expect.
//if result has err value then the program will crash desplaying
//the "---" message inside expect method.
//if i dont really use expect, i'll get a warnign while debugging
//this structure is called "error-handling" code 

		println!("you guessed: {guess}");
//{} is a placeholder[crab pincers]. it pinces a value. 
//its like %d, so lets say i need to print y+2
//let mut y=2; println("x is {},y+2};
}
