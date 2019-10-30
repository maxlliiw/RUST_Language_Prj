/*Programming Languages Language Learning Project 2
 *Authors: William Shaw, Peter Costantino, Ryan Rice
 *Version: 0.1a (10/21/19)
 *Description: Takes a sequence of user inputed numbers and computes the sum of the sequence of numbers. Exits when 'q' in entered. Uses 32 bit floating point data types to take in numbers. Can cause floating point precision error.
 */ 
use std::io;
fn main() {
	let mut run = true;
	println!("Enter a sequence of numbers... \nType 'q' to return sum");
	
	
	let mut sum: f32 = 0.0;
	while run {
		let mut a = String::new();
		io::stdin().read_line(&mut a)
			.expect("Failed to read line!");
		if a.trim() == "q" {
			println!("Sum: {}", sum);
			run = false;
		} else {
			let a: f32 = a.trim().parse()
				.expect("Please type a number!");
			sum = sum + a;
		};
	}
}
