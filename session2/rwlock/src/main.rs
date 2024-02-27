use once_cell::sync::Lazy;
use std::io::stdin;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    thread::spawn(|| loop {
        println!("Current users (in a thread)");
        let users = USERS.read().unwrap();
        println!("{users:?}");
        thread::sleep(Duration::from_secs(3));
    });

    loop {
        println!("Enter a name to add to the user list (or q to quit)");
        let input = read_line();
        if input == "q" {
            break;
        }
        let mut lock = USERS.write().unwrap();
        lock.push(input);
    }
}
