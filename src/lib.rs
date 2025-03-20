use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

// New builder struct
pub struct ThreadPoolBuilder {
    size: Option<usize>,
    name_prefix: Option<String>,
}

impl ThreadPoolBuilder {
    pub fn new() -> Self {
        ThreadPoolBuilder {
            size: None,
            name_prefix: Some("worker".to_string()),
        }
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn name_prefix(mut self, prefix: String) -> Self {
        self.name_prefix = Some(prefix);
        self
    }

    pub fn build(self) -> Result<ThreadPool, &'static str> {
        let size = match self.size {
            Some(s) if s > 0 => s,
            Some(_) => return Err("Thread pool size must be greater than zero"),
            None => return Err("Thread pool size must be specified"),
        };

        let prefix = self.name_prefix.unwrap_or_else(|| "worker".to_string());
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), format!("{}-{}", prefix, id)));
        }

        Ok(ThreadPool { workers, sender })
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), format!("worker-{}", id)));
        }

        ThreadPool { workers, sender }
    }
    
    // Factory method that uses the builder
    pub fn build(size: usize) -> Result<ThreadPool, &'static str> {
        ThreadPoolBuilder::new().size(size).build()
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, name: String) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("{} got a job; executing.", name);
            job();
        });
        Worker { id, thread }
    }
}