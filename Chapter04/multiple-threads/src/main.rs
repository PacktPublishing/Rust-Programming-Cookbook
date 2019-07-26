use std::thread;


///
/// Doubles each element in the provided chunks in parallel and returns the results.
/// 
fn parallel_map(data: Vec<Vec<i32>>) -> Vec<thread::JoinHandle<Vec<i32>>> {
    data.into_iter()
        .map(|chunk| thread::spawn(move || chunk.into_iter().map(|c| c * 2).collect()))
        .collect()
}

fn main() {

    // Prepare chunked data
    let data = vec![vec![1, 2, 3], vec![4, 4, 5], vec![6, 7, 7]];

    // work on the data in parallel
    let results: Vec<i32> = parallel_map(data.clone())
        .into_iter() // an owned iterator over the results
        .flat_map(|thread| thread.join().unwrap()) // join each thread
        .collect(); // collect the results into a Vec

    // flatten the original data structure
    let data: Vec<i32> = data.into_iter().flat_map(|e| e).collect();

    // print the results
    println!("{:?} -> {:?}", data, results);
}
