use std::{ thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Message>
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Terminating all workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker  in &mut self.workers {
            println!("Shutting down worker{}", worker.id);
            
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
//struct Job;
type Job = Box<dyn FnOnce() + Send+'static>;

enum Message{
    NewJob(Job),
    Terminate
}
impl ThreadPool{
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        
        //for creating channel between threads
        let (sender, reciever ) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        
        let reciever = Arc::new(Mutex::new(reciever));
        for id in 0..size{
           workers.push(Worker::new(id, Arc::clone(&reciever)));
        }
        ThreadPool{workers, sender}
    }
     
    //exicute is built with the same interface/parameters as the thread::spawn function
    pub fn execute<F>(&self, f:F)
     where 
     F:FnOnce()+Send +'static
     {
       let job = Box::new(f);
       self.sender.send(Message::NewJob(job)).unwrap();
     }
}

struct  Worker {
    id:usize,
    thread:Option<thread::JoinHandle<()>> 
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            //there's no unlock for mutex if its lock it'll automatically unlock
            let message = receiver.lock().unwrap().recv().unwrap();

            match message{
                Message::NewJob(job)=>{
                    println!("Worker {id} got a job; executing.");
                 job(); 
                },
                Message::Terminate=>{
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
           
        });

        Worker { id, thread: Some(thread) }
    }
}