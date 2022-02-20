// Building the ThreadPool Struct Using Compiler Driven Development

pub struct ThreadPool1;

impl ThreadPool1 {
    pub fn new_1(size: usize) -> ThreadPool1 {
        ThreadPool1
    }

    pub fn execute_1<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

// Validating the Number of Threads in new

impl ThreadPool1 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_2` function will panic if the size is zero.
    pub fn new_2(size: usize) -> ThreadPool1 {
        assert!(size > 0);

        ThreadPool1
    }

    pub fn execute_2<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

// Creating Space to Store the Threads

use std::thread;

pub struct ThreadPool3 {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool3 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_3` function will panic if the size is zero.
    pub fn new_3(size: usize) -> ThreadPool3 {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool3 { threads }
    }

    pub fn execute_3<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

// A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread

pub struct ThreadPool4 {
    workers: Vec<Worker4>,
}

impl ThreadPool4 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_4` function will panic if the size is zero.
    pub fn new_4(size: usize) -> ThreadPool4 {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker4::new_4(id))
        }

        ThreadPool4 { workers }
    }

    pub fn execute_4<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker4 {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker4 {
    fn new_4(id: usize) -> Worker4 {
        let thread = thread::spawn(|| {});

        Worker4 { id, thread }
    }
}

// Sending Requests to Threads via Channels

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool5 {
    workers: Vec<Worker5>,
    sender: mpsc::Sender<Job5>,
}

struct Job5;

impl ThreadPool5 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_5` function will panic if the size is zero.
    pub fn new_5(size: usize) -> ThreadPool5 {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker5::new_5(id, Arc::clone(&receiver)))
        }

        ThreadPool5 { workers, sender }
    }

    pub fn execute_5<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker5 {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker5 {
    fn new_5(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job5>>>) -> Worker5 {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker5 { id, thread }
    }
}

// Implementing the execute Method

pub struct ThreadPool6 {
    workers: Vec<Worker6>,
    sender: mpsc::Sender<Job6>,
}

type Job6 = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool6 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_6` function will panic if the size is zero.
    pub fn new_6(size: usize) -> ThreadPool6 {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker6::new_6(id, Arc::clone(&receiver)))
        }

        ThreadPool6 { workers, sender }
    }

    pub fn execute_6<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker6 {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker6 {
    fn new_6(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job6>>>) -> Worker6 {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker6 { id, thread }
    }
}
pub struct ThreadPool7 {
    workers: Vec<Worker7>,
    sender: mpsc::Sender<Job7>,
}

type Job7 = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool7 {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new_7` function will panic if the size is zero.
    pub fn new_7(size: usize) -> ThreadPool7 {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker7::new_7(id, Arc::clone(&receiver)))
        }

        ThreadPool7 { workers, sender }
    }

    pub fn execute_7<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker7 {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker7 {
    fn new_7(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job7>>>) -> Worker7 {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker7 { id, thread }
    }
}
