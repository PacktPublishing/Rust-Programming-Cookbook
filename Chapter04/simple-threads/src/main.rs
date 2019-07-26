use std::thread;
use std::time::Duration;

///
/// Starts a thread that waits for 3 seconds before returning. Prints stuff.
/// 
fn start_no_shared_data_thread() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        // since we are not using a parent scope variable in here
        //   no move is required
        println!("Waiting for three seconds.");
        thread::sleep(Duration::from_secs(3));       
        println!("Done")
    })
}


///
/// Starts a thread moving the function's input parameters
/// 
fn start_shared_data_thread(a_number: i32, a_vec: Vec<i32>) -> thread::JoinHandle<Vec<i32>> {
    thread::spawn(move || {
    // thread::spawn(|| {
        print!("   a_vec ---> [");
        for i in a_vec.iter() {
            print!(" {} ", i);
        }
        println!("]");
        println!("   A number from inside the thread: {}", a_number);
        a_vec // let's return ownership
    })
}

fn main() {
    let no_move_thread = start_no_shared_data_thread();
    
    // do some work in between
    for _ in 0..10 {
        print!(":");
    }

    println!("Waiting for the thread to finish ... {:?}", no_move_thread.join());

    let a_number = 42;
    let a_vec = vec![1,2,3,4,5];

    let move_thread = start_shared_data_thread(a_number, a_vec);

    println!("We can still use a Copy-enabled type: {}", a_number);    
    println!("Waiting for the thread to finish ... {:?}", move_thread.join());
}