use std::thread;
use std::time::Duration;
use std::sync::{Mutex,Arc};
use std::rc::Rc;
fn main(){
   //thread_basics()
   mutex()
}

fn mov_closure(){
    //the move closures is used to move the data into the thread
    let v = vec![1,2,3];

    let handle  = thread::spawn(move ||{
        println!("the vector is {:?}",v);
    });
    handle.join().unwrap()
}

fn  thread_basics(){
    let handle = thread::spawn(|| {
        for  i in 1..10{
            println!("spawned thread says: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //the below commands waits untill the thread handle is finished completly
    handle.join().unwrap();


    for i in 1..8{
        println!("main thread says:{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    //when the main thread sleeps all threads are shut down
}

fn mutex(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 3;
    }
    println!("m= {:?}",m);
}

fn channel(){
    //Arc is same as the Rc function but Rc is not safe in threads creating multiple copies
    let counter  =  Arc::new(Mutex::new(0));
    let mut  handles = vec![];
    for i in 0..10{
        let counter  = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num  = counter.lock().unwrap();
            *num+=1;
            println!("{i}, {num}")
        });
        handles.push(handle);

    }
    for handle in handles{
        handle.join().unwrap();
    }

    println!("Reslut {}", *counter.lock().unwrap());
}