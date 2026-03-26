use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, Thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>
}
type Job = Box<dyn AsyncFnOnce() + Send + 'static >;
impl ThreadPool{
    pub fn new(size: usize) -> Self{
        let mut threads = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)))
        }
        Self{
            workers: threads,
            sender: Some(sender),
        }
    }
    pub async fn execute<F: AsyncFnOnce() + Send + 'static>(&self, job: F) {
        let job = Box::new(job);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..){
            worker.thread.join().unwrap();
        }
    }
}

struct Worker{
    id: usize,
    thread: JoinHandle<()>,
}
impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>)-> Self{
        let thread = thread::spawn(move||{
            loop{
                let message = receiver.lock().unwrap().recv();
                match message{
                    Ok(job) => {
                        println!("{id} executing job");
                        job()
                    }
                    Err(error) => {
                        println!("error: {error}, shutting down");
                        break;
                    }
                }
            }
        });
        Self{
            id,
            thread,
        }
    }
}