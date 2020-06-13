use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};

struct Worker {
    // id: usize,
    // w_thread: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize, rc: Arc<Mutex<Receiver<Job>>> ) -> Worker {
        // let w_thread = thread::spawn(move || {
        thread::spawn(move || loop {
            let job = rc.lock().unwrap().recv().unwrap();
            job();
            println!("thread {} is running", id);
        });
        // Worker {id, w_thread}
        Worker{}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    // workers: Vec<Worker>,
    sd: Sender<Job>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sd, rc) = mpsc::channel();
        let rc = Arc::new(Mutex::new(rc));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rc))) ;
        }

        // ThreadPool { workers, sd }
        ThreadPool { sd }
    }

    // send closure to worker
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        self.sd.send(Box::new(f)).unwrap();
    }
}

