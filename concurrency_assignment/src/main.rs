
/* Assignment 3
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create channel
        let (sender, receiver) = mpsc::channel();

        // Share receiver safely across threads
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        // Create workers
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // Send terminate message to all workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // Join all worker threads
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);

    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }

    println!("Main thread waiting for tasks to complete...");
}
*/

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel::<i32>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // Spawn producers
    for id in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let count = ITEM_COUNT / NUM_PRODUCERS;
        handles.push(thread::spawn(move || producer(id, tx_clone, count)));
    }

    // Spawn consumers
    for id in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        handles.push(thread::spawn(move || consumer(id, rx_clone)));
    }

    // Drop our tx clone so only producer clones remain
    drop(tx);

    // Wait for producers to finish, then send termination signals
    // (we join all handles at the end, but producers finish first naturally)
    // Send one termination signal per consumer after producers finish
    // We re-create a sender for this; instead, collect producer handles separately:
    // Simpler: just wait for all handles
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    for _ in 0..item_count {
        let num: i32 = rand::random::<i32>().abs() % 100;
        println!("Producer {} sending: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} done", id);
    // tx dropped here; when all producers drop, channel closes
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let msg = rx.lock().unwrap().recv();
        match msg {
            Ok(TERMINATION_SIGNAL) => {
                println!("Consumer {} received termination signal", id);
                break;
            }
            Ok(val) => println!("Consumer {} processed: {}", id, val),
            Err(_) => {
                // Channel closed (all producers done)
                println!("Consumer {} exiting (channel closed)", id);
                break;
            }
        }
    }
}