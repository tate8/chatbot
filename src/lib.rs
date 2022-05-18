use std::thread;
use std::sync::mpsc;
// let multiple workers own the receiver
use std::sync::Arc;
// ensure that only one worker gets a job from the receiver at a time
use std::sync::Mutex;


/*
    A thread pool is a group of spawned threads that are waiting and ready to handle a task
*/
pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl ThreadPool {
    // usize for unsigned params (negative threads doesn't make sense)
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size
        {
            // clone the Arc to bump the reference count so the workers can share ownership of the receiving end
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool
        {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where // FnOnce because only executed once, Send for transfer between threads, static lifetime
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers
        {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        
        for worker in &mut self.workers
        {
            println!("Shutting down worker {}", worker.id);

            // call take on the Option value to move thread out of worker
            // the take method on Option takes the Some variant out and leaves None in its place
            if let Some(thread) = worker.thread.take()
            {
                thread.join().unwrap();
            }
        }
    }
}


/*
    Responsible for sending code from the ThreadPool to a thread
*/
pub struct Worker
{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker
    {
        let thread = thread::spawn(move || {
            loop
            {
                // call lock on the receiver to acquire the mutex
                // call recv to receive a Job from the channel
                let message = receiver.lock().unwrap().recv().unwrap();
                match message
                {
                    Message::NewJob(job) =>
                    {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });
        Worker
        {
            id,
            thread: Some(thread),
        }
    }
}


type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox
{
    fn call_box(self: Box<Self>);
}

// implement the FnBox trait for any type F that implements the FnOnce() trait
// this means that any FnOnce() closures can use our call_box method
impl<F: FnOnce()> FnBox for F
{
    fn call_box(self: Box<F>)
    {
        // move the closure out of the Box<T> and call the closure
        (*self)();
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

// TODO: Create HTTP Request and Response containers
// HTTP Response needs a HEADER where put content type styles or javascript etc