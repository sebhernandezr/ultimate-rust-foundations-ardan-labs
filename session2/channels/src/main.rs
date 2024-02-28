use std::sync::mpsc;
use std::thread;

enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let handle = thread::spawn(move || {
        while let Ok(command) = rx.recv() {
             match command {
                 Command::SayHello => println!("Hello"),
                 Command::Quit => {
                     println!("Quitting");
                     break;
                 }
             }    
        }
    });

    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap();
    }
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
