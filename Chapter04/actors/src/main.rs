
use actix::prelude::*;
use std::thread;
use std::time::Duration;
use rand::prelude::*;

const N_THREADS : usize = 3;

///
/// A mock sensor function
/// 
fn read_sensordata() -> f32 {
     random::<f32>() * 10.0
}

#[derive(Debug, Message)]
struct Sensordata(pub u64, pub f32);

struct DBWriter;

impl Actor for DBWriter {
    type Context = SyncContext<Self>;
}

impl Handler<Sensordata> for DBWriter {
    type Result = ();

    fn handle(&mut self, msg: Sensordata, _: &mut Self::Context) -> Self::Result {

        // send stuff somewhere and handle the results
        println!("  {:?}", msg);
        thread::sleep(Duration::from_millis(300));
    }
}

fn main() -> std::io::Result<()> {
    System::run(|| {
        println!(">> Press Ctrl-C to stop the program");
        // start multi threaded actor host (arbiter) with 2 threads
        let sender = SyncArbiter::start(N_THREADS, || DBWriter);
      
        // send messages to the actor 
        for n in 0..10_000 {
            let my_timestamp = n as u64;
            let data = read_sensordata();
            sender.do_send(Sensordata(my_timestamp, data));
        }
    })
}
