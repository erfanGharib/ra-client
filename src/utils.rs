use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_with_delay<F>(
    func: F, 
    delay_time: u64
) where F: Fn() -> () {
    let delay = Arc::new(Mutex::new(0));
    let delay_clone = Arc::clone(&delay);

    // Spawn a new thread that will wait for @delay_time seconds
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(delay_time));
        *delay_clone.lock().unwrap() += 1;
    });
    
    // Wait for the delay to complete
    thread::sleep(Duration::from_secs(1));
    
    // Kill the spawned thread
    handle.join().unwrap();
    
    func();
}