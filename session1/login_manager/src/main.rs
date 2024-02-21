use authentication::get_users;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add user.
    Add {
        /// Logon name.
        username: String,
        /// Password (plaintext).
        password: String,
        /// Optional - mark as an admin.
        admin: Option<bool>,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "password");
    println!("{:-<40}", "");

    let users = get_users();
    users.iter().for_each(|(_, user)| {
        println!("{:<20}{:<20?}", user.username, user.role);
    })
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => list_users(),
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => println!("Add a user"),
        None => println!("Run with --help to see instructions."),
    }
}
