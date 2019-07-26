use a_lib::stringify;
use rand::prelude::*;

fn main() {
    println!("{{ \"values\": {}, \"sensor\": {} }}", stringify(&vec![random::<f64>(); 6]), stringify(&"temperature"));
}
