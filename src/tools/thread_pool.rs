use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    End,
    NewJob(Job),
}

pub struct PoolJoinHandle<T> {
    receiver: Receiver<T>,
}

impl<T> PoolJoinHandle<T> {
    pub fn join(&self) -> T {
        self.receiver.recv().unwrap()
    }
}

pub struct ThreadPool {
    sender: Sender<Message>,
    pub handlers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num: u32) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>();
        let rex2 = Arc::new(Mutex::new(receiver));

        let mut handlers = vec![];
        for _ in 0..num {
            let receiver = Arc::clone(&rex2);
            let th = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::End => {
                        break;
                    }
                    Message::NewJob(job) => {
                        job();
                    }
                }
            });
            handlers.push(th);
        }

        ThreadPool { sender, handlers }
    }
    pub fn execute<T: 'static + Send>(
        &self,
        f: impl FnOnce() -> T + 'static + Send,
    ) -> PoolJoinHandle<T> {
        let (sender, receiver) = mpsc::channel::<T>();

        let tx_clone = sender.clone();
        let f1 = move || {
            let res = f();
            let _ignored_result = tx_clone.send(res);
        };
        let _ = self.sender.send(Message::NewJob(Box::new(f1)));

        PoolJoinHandle { receiver }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.handlers {
            let _ignored_result = self.sender.send(Message::End);
        }

        for item in self.handlers.pop().into_iter() {
            let _ = item.join();
        }
    }
}
