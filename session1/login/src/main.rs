use authentication::{greet_user, login, read_line, LoginAction};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username");
        let username = read_line();

        println!("Enter your password");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                println!("{}. You are a {:?}", greet_user(&username), role);
                break;
            }
            Some(LoginAction::Denied) => {}
            None => println!("New user"),
        }

        tries += 1;
        println!(
            "Incorrect username or password. You have {} tries left",
            3 - tries
        );
        if tries >= 3 {
            println!("Too many tries");
            break;
        }
    }
}
