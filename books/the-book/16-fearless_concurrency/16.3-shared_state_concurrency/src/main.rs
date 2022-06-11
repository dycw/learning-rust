// The API of Mutex<T>

use std::sync::Mutex;

fn the_api_of_mutex_t() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// Sharing a Mutex<T> Between Multiple Threads
// Multiple Ownership with Multiple Threads

use std::sync::Arc;
use std::thread;

fn atomic_reference_counting_with_arc_t() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    atomic_reference_counting_with_arc_t();
}
