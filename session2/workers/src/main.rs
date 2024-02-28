use std::sync::mpsc;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn hi() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
    });
    
    let job = || println!("Hi from a closure");
    let job2 = || {
        for i in 0..10 {
            println!("{i}");
        }
    };

    tx.send(Command::Run(Box::new(job))).unwrap();
    tx.send(Command::Run(Box::new(job2))).unwrap();
    tx.send(Command::Run(Box::new(hi))).unwrap();
    tx.send(Command::Run(Box::new(|| println!("I'm in a box")))).unwrap();
    tx.send(Command::Quit).unwrap();
    
    handle.join().unwrap();
}
