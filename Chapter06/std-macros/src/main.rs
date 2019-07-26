
#[derive(Debug)]
pub struct MyStruct(usize);


fn main() {
    println!("Hello, world!");

    println!("a vec: {:?}", vec!{1, 2, 3});
    println!("concat: {}", concat!(0, 'x', "5ff"));
    println!("MyStruct stringified: {}", stringify!(MyStruct(10)));
    println!("some random word stringified: {}", stringify!(helloworld));

    println!("Running on Windows? {}", cfg!(windows));
    println!("From a file: {}", include_str!("a.txt"));
    println!("$PATH: {:?}", option_env!("lala")); // evaluated at compile time!

    eprintln!("Oh no!");
    debug_assert!(true);
}