use std::thread;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::channel;


fn noop<T>(_: T) {}

fn main() {
    let (sender, receiver) = channel::<usize>();

    thread::spawn(move || {
        let thread_local_read_only_clone = sender.clone();
        noop(thread_local_read_only_clone);
    });

    
    let b = Arc::new(Rc::new(vec![]));
    thread::spawn(move || {
        let thread_local_read_only_clone = b.clone();
        noop(thread_local_read_only_clone);
    });
    /* This won't compile:

    let c = Arc::new(receiver);
    thread::spawn(move || {
        noop(c.clone());
    });*/
}