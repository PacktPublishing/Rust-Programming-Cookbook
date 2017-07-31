// Task : To explain assignment operations in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

// Primitive libraries in rust
use std::{i8,i16,i32,i64,u8,u16,u32,u64,f32,f64,isize,usize};
use std::io::stdin;

fn main() {
	println!("Understanding assignment");	
	// Complier will automatically figure out the datatype if not mentioned
	// Cannot change the value
	let num =10;
	println!("Num is {}", num);

	// immutuable can change the value
	let age: i32 =40;
	println!("Age is {}", age);

}