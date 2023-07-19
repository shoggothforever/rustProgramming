use std::sync::{mpsc, Mutex,Arc};
use std::thread;
use std::time::Duration;
use mpsc::channel;
use std::sync::mpsc::RecvError;
use std::process;


fn demo_spawn(){
    let handle1 =thread::spawn(||{
        for i in 1..10{
            println!("number from the spawned thread1 :{i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    let handle2 =thread::spawn(||{
        for i in 1..20{
            println!("number from the spawned thread2 :{i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    let handle3 =thread::spawn(||{
        for i in 1..30{
            println!("number from the spawned thread3 :{i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("number from the main thread :{i}");
        thread::sleep(Duration::from_millis(1));
    }
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}
fn demo_channel(){
    let (pc,rc)=mpsc::channel();
    let pcc=mpsc::Sender::clone(&pc);
    let sender1=thread::spawn(move||{
        for i in 1..100{

            pc.send("hello").unwrap();
            println!("send {i} msg  from sender1 ");
            thread::sleep(Duration::from_millis(100));
        }
    });

    let sender2=thread::spawn(move||{
        for i in 1..100{

            pcc.send("hello").unwrap();
            println!("send {i} msg  from sender2 ");
            thread::sleep(Duration::from_millis(100));
        }
    });
    let mut i =0;
    let recever=thread::spawn(move||{
        for recv in rc{
            i+=1;
            println!("received {i} msg {recv} from channel ");
        }
    });
    recever.join();
    println!("channle closed  ")

}
fn demo_mutex(){
    let num_=0;
    let m=Arc::new(Mutex::new(num_));
    let am1=Arc::clone(&m);
    let am2=Arc::clone(&m);
    let t1=thread::spawn(move||{
        {
            let mut num = am1.lock().unwrap();
            *num += 1;
            println!("add one to num from t1");
        }
    });
    let t2=thread::spawn(move||{
        let mut num=am2.lock().unwrap();
            *num+=1;
        println!("add one to num from t2");

    });

    println!("{:?}", m);
}
fn main() {
demo_mutex();
}
