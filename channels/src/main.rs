use std::sync::mpsc;
use std::thread;
use std::time::Duration;
    
fn main() {
    // tx: transmitter, rx: receiver
    let (tx, rx) = mpsc::channel();
    // Making a second transmitter to the same reciever.
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let var = String::from("some stuff");
        tx.send(var).unwrap();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // we have sent ownership of val to the thread holding rx.
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("found"),
            String::from("here"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // we have sent ownership of val to the thread holding rx.
    });


    // Single recieve
    println!("Got: {}", rx.recv().unwrap());

    // Recieve all
    for recieved in rx {
        println!("Got: {recieved}");
    }
}
