/*Programming Languages Language Learning Project 1
 *Authors: William Shaw, Peter Costantino, Ryan Rice
 *Version: 0.1a (10/14/19)
 *Description: Takes two user entered unsigned 32 bit integers and adds them to create a sum 
 */ 
use std::io;
fn main() {
	println!("Enter two numbers...");
	let mut a = String::new();
	let mut b = String::new();
	
	io::stdin().read_line(&mut a)
		.expect("Failed to read line!");
	io::stdin().read_line(&mut b)
		.expect("Failed to read line!");
	
	let a: f64 = a.trim().parse()
		.expect("Please type a number!");
	let b: f64 = b.trim().parse()
		.expect("Please type a number!");
	
	println!("Sum: {}", (a+b));
}
