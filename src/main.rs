use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::env;
static NTHREADS: i32 = 5;


fn mpsc_generic() {

    //channels have two end points
    //mpsc - multiple producers single consumer
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    //maintain a list of thread handler
    let mut thread_handlers = Vec::new();


    //spawn and run NTHREADS
    for id in 0..NTHREADS {

        //clone the tx to pass to thread
        let thread_tx = tx.clone();

        //spawn thread
        let handler = thread::spawn(move || {

            //send id to Parent
            thread_tx.send(id).unwrap();
            println!("thread { } finished", id);
        });

        // add thread to handlers list
        thread_handlers.push(handler);
    }


    //Receive data from child threads
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        //blocking if nothing in the channel
        ids.push(rx.recv());
    }

    // join the thread handles
    for trd in thread_handlers {
        trd.join().expect("Thread panicked");
    }

    println!("{:?}", ids);
}

type Job = Box<dyn FnOnce() + Send + 'static>;

fn mpsc_to_spmc() {
    //initialise a channel with the dynamic box type
    let (tx, rx) = mpsc::channel::<Job>();

    /*in the mpsc channels receiver side can only be acquired by one owner
       with that in mind, we get a reference counted pointer with a mutex
       now the thread needs to acquire a lock before receiving*/

    let rx_rc = Arc::new(Mutex::new(rx));


    //spawn threads
    let mut thread_handlers = Vec::with_capacity(NTHREADS as usize);
    for id in 0..NTHREADS {
        let rx_cc = Arc::clone(&rx_rc);
        let handle = thread::spawn(move || loop {
            let job = match rx_cc.lock().expect("can't acquire lock of receiver").recv(){
                Ok(job) => {
                    println!("Thread id {id} received function from main thread");
                    job
                },
                Err(_) => break,
            };

            job();

        });
        thread_handlers.push(handle);

    }
    let send_func = |id: i32|{
        println!("Main thread sending {id}");
    };

    for i in 0..100 {
        tx.send(Box::new(move ||
            send_func(i)
        )).unwrap();
    }

    drop(tx);

    //join all threads
    for trd in thread_handlers{
        trd.join().expect("Thread panicked");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: no arguments provided");
        return Err("No arguments provided".into());
    }

    let mpsc_type = &args[1];

    if mpsc_type == "mpsc" {
        println!("Multi Producer Single consumer ....");
        mpsc_generic();
    }else if mpsc_type == "scmp" {
        println!("Single Producer Multi Consumer ...");
        mpsc_to_spmc();
    }

    Ok(())

}
