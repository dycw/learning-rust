// Creating a New Thread with spawn

use std::thread;
use std::time::Duration;

fn creating_a_new_thread_with_spawn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Waiting for All Threads to Finish Using join Handles

fn waiting_for_all_threads_to_finish_using_join_handles_1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn waiting_for_all_threads_to_finish_using_join_handles_2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Using move Closures with Threads

fn using_move_closures_with_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));

    handle.join().unwrap();
}

fn main() {
    using_move_closures_with_threads();
}
