use std::sync::{mpsc,Arc,Mutex};
use std::thread;
// why we need threadpool

/*
Thread-per-connection works for small servers but doesn’t scale well (thousands of clients = thousands of threads).
So We Create a Thread Pool.

Why Thread Pool?

Thread per connection → works fine for small loads but is dangerous at scale.

Every thread::spawn:

Allocates a stack (2MB default per thread on most OSes).

Requires OS scheduling overhead.

If 10,000 clients hit your server at once → your system may run out of memory or just crawl.

Thread Pool Fix:

Create N worker threads at startup.

Use a queue (channel) to store incoming jobs.

Workers keep pulling jobs from the queue and executing them.

Max threads = N → no unbounded memory use.
*/
pub struct  ThreadPool{
    workers: Vec<Worker>,
    sender:  mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    // Create a new ThreadPool with `size` worker threads.
    pub fn new(size : usize) -> ThreadPool{
        assert!(size > 0);

        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }

    }
}


struct Worker{
    id : usize,
    thread : thread::JoinHandle<()>
}

impl Worker{
    fn new(id : usize, receiver : Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        // you are creating here a thread.
        let thread = thread :: spawn(move || loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing...");
            job();
        });

        Worker { id,thread}
    }
}