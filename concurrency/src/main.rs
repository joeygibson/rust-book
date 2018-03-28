use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi. Number {} from thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi. Number {} from main", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vec: {:?}", v);
    });

    handle.join();

    channel_exercise();
    multi_value_channel();
    mutexes();
}

fn channel_exercise() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello");

        tx.send(val).unwrap();

        // won't work
//        println!("VAL: {}", val);
    });

    let received = rx.recv().unwrap();

    println!("Got: {}", received);
}

fn multi_value_channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Mux GOT: {}", received);
    }
}

fn mutexes() {
//    let counter = Mutex::new(0);
//    let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}