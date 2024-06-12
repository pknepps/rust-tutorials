//! # ThreadPool
//! A crate containing the TreadPool struct, which limits the number of open 
//! threads to prevent server attacks such as DDOSing.

use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread,
};

/// A pool of threads. An unlimited number of threads can be added to a
/// pool but only a finite number, specified by the user, can be ran at one
/// time.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Errors which can occur when trying to create a new ThreadPool.
pub enum PoolCreationError{
    /// If the given size of the ThreadPool is not positive.
    InvalidSize,
}

impl ThreadPool {
    
    /// Creates a new ThreadPool with the given size.
    ///
    /// ## Parameters:
    /// - size: The number of threads in the pool which are allowed to run at
    ///         one time.
    ///
    /// ## Panics
    /// - If *size* is non-positive.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        Self::construct(size)
    }

    /// Creates a new ThreadPool with the given size.
    ///
    /// ## Parameters:
    /// - size: The number of threads which are allowed to run at one time.
    ///
    /// ## Errors
    /// - PoolCreationError::InvalidSize: If *size* is not a positive number.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::InvalidSize);
        }
    
        Ok(Self::construct(size))
    }

    // Helper for constructors.
    fn construct(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.sender.as_ref().unwrap().send(job).unwrap();   
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
        
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
            
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                },
            }
        });
    
        Worker { 
            id, 
            thread: Some(thread),
        }
    }
}
