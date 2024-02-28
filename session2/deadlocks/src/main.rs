use std::sync::Mutex;
use std::thread;

static MY_SHARE: Mutex<u32> = Mutex::new(0);

fn poisoner() {
    let mut lock = MY_SHARE.lock().unwrap();
    *lock += 1;
    panic!("The poisoner strikes");
}

// fn main() {
//     let lock = MY_SHARE.lock().unwrap();
//     drop(lock);
//     if let Ok(_lock) = MY_SHARE.try_lock() {
//         println!("Got the lock");
//     } else {
//         println!("No lock");
//     }
// }

fn main() {
    let handle = thread::spawn(poisoner);
    println!("Trying to return from the thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARE.lock();
    println!("{lock:?}");

    let recovered_data = lock.unwrap_or_else(|poisoned| {
        println!("The mutex was poisoned");
        poisoned.into_inner()
    });
    println!("{recovered_data}")
}
