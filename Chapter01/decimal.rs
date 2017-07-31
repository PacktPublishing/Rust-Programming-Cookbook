// Task : To explain assignment operations in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

fn main(){
	// Prints the first 2 numbers after the decimal points
	println!("{:.2}",1.2345 );

	println!("================");
	// print the binary hex and octal format
	println!("B: {:b} H: {:x} O: {:o}",10,10,10 );

	println!("================");
	// Shifts
	println!("{ten:>ws$}",ten=10, ws=5 );
	println!("{ten:>0ws$}",ten=10, ws=5 );
}
