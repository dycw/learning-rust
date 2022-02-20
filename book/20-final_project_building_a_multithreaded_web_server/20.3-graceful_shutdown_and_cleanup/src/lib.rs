// Imlementing the Drop Trait on ThreadPool

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool1 {
    workers: Vec<Worker1>,
    sender: mpsc::Sender<Job1>,
}

type Job1 = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool1 {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool1 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_1` function will panic if the size is zero.
    pub fn new_1(size: usize) -> ThreadPool1 {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker1::new_1(id, Arc::clone(&receiver)))
        }

        ThreadPool1 { workers, sender }
    }

    pub fn execute_1<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker1 {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker1 {
    fn new_1(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job1>>>) -> Worker1 {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker1 {
            id,
            thread: Some(thread),
        }
    }
}

// Signaling to the Threads to Stop Listening for Jobs

enum Message2 {
    NewJob(Job2),
    Terminate,
}

pub struct ThreadPool2 {
    workers: Vec<Worker2>,
    sender: mpsc::Sender<Message2>,
}

type Job2 = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool2 {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message2::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool2 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_2` function will panic if the size is zero.
    pub fn new_2(size: usize) -> ThreadPool2 {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker2::new_2(id, Arc::clone(&receiver)))
        }

        ThreadPool2 { workers, sender }
    }

    pub fn execute_2<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message2::NewJob(job)).unwrap();
    }
}

struct Worker2 {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker2 {
    fn new_2(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message2>>>) -> Worker2 {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message2::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message2::Terminate => {
                    println!("Worker {} was told to terminate", id);

                    break;
                }
            }
        });

        Worker2 {
            id,
            thread: Some(thread),
        }
    }
}
