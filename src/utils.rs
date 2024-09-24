use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_with_delay<F>(
    func: F, 
    delay_time: u64
) where F: Fn() -> () {
    let delay = Arc::new(Mutex::new(0));
    
    // Spawn a new thread that will wait for 5 seconds
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(delay_time));
        *delay.lock().unwrap() += 1;
    });
    
    func();

    // Main thread: do some work
    println!("Doing some work...");
    
    // Wait for the delay to complete
    thread::sleep(Duration::from_secs(1));
    

    // Check if the delay has completed
    println!("Delay completed? {}", delay.lock().unwrap());
    
    // Kill the spawned thread
    handle.join().unwrap();
}