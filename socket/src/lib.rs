
use std::{fs, sync};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc,mpsc, Mutex};
use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;

pub fn handle_stream(mut stream:TcpStream){
    let mut buffer =[0;512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}",String::from_utf8_lossy(&buffer[..]));
    let get=b"GET /home HTTP/1.1\r\n";
    let sleep=b"GET /sleep HTTP/1.1\r\n";
    let (status,filename)=if buffer.starts_with(get){
        ("HTTP/1.1 200 OK","home.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_millis(10000));
        ("HTTP/1.1 200 OK","home.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };
    let content=fs::read_to_string(filename).unwrap();
    let response=format!("{}\r\n\r\n{}",status,content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
pub struct ThreadPool{
 workers:Vec<Worker>,
 sender:Option<mpsc::Sender<Job>>,
 size:usize,
}
impl Drop for ThreadPool{
    fn drop(&mut self){
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread)=worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
struct Worker{
    id:usize,
    thread:Option<JoinHandle<()>>
}
type Job=Box<dyn FnOnce()+Send+'static>;
// struct Job{
//
// }
impl ThreadPool{
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) ->Self{
        assert!(size>0);
        let mut workers=Vec::with_capacity(size);
        let (sender,receiver)=mpsc::channel();
        let receiver=sync::Arc::new(Mutex::new(receiver));
        for id in 1..=size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{workers,sender:Some(sender),size}
    }
    pub fn execute<F>(&self,f:F)
    where F:FnOnce()+Send+'static
    {
        let job=Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Worker{
    fn new(id :usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Self
    {
        let thread=thread::spawn(move||loop{
            let message=receiver.lock().expect("锁竞争失败").
                recv();

            match message{
                Ok(job) =>{
                    println!("Worker {id} got a message; executing.");
                    job();
                },
                Err(_)=>{
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker{id,thread:Some(thread)}
    }
}